use crate::auth::KalshiSigner;
use crossbeam_channel::Sender;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, tungstenite::client::IntoClientRequest};

pub async fn run(
    log_tx: Sender<Value>, 
    tickers: Vec<String>, 
    signer: Arc<KalshiSigner>, 
    debug: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let (key_id, sig, ts) = signer.get_auth_headers();
    let mut req = url::Url::parse("wss://api.elections.kalshi.com/trade-api/ws/v2")?.into_client_request()?;
    
    let h = req.headers_mut();
    h.insert("KALSHI-ACCESS-KEY", key_id.parse()?);
    h.insert("KALSHI-ACCESS-SIGNATURE", sig.parse()?);
    h.insert("KALSHI-ACCESS-TIMESTAMP", ts.parse()?);

    let (mut ws, _) = connect_async(req).await?;

    // Subscribing according to V2 spec: cmd + params
    let sub_msg = json!({
        "id": 1,
        "cmd": "subscribe",
        "params": {
            "channels": ["orderbook_delta", "ticker"],
            "market_tickers": tickers
        }
    });
    ws.send(Message::Text(sub_msg.to_string())).await?;

    let mut seq_map: HashMap<String, u64> = HashMap::new();

    while let Some(Ok(msg)) = ws.next().await {
        if let Ok(text) = msg.to_text() {
            if debug { println!("DEBUG: {}", text); }
            if let Ok(json) = serde_json::from_str::<Value>(text) {
                
                // Extract ticker from the inner 'msg' object
                let ticker = match json["msg"]["market_ticker"].as_str() {
                    Some(t) => t.to_string(),
                    None => continue,
                };

                let msg_type = json["type"].as_str().unwrap_or("");
                
                // Only orderbook_delta has strict sequence requirements
                if msg_type == "orderbook_delta" {
                    let seq = json["seq"].as_u64().unwrap_or(0);
                    let last = seq_map.entry(ticker.clone()).or_insert(0);

                    if *last != 0 && seq != *last + 1 {
                        println!("⚠️ GAP on {}: Expected {}, got {}", ticker, *last + 1, seq);
                        // Surgical Resubscribe
                        let unsub = json!({"id": 10, "cmd": "unsubscribe", "params": {"market_tickers": [ticker.clone()]}});
                        let sub = json!({"id": 11, "cmd": "subscribe", "params": {"channels": ["orderbook_delta"], "market_tickers": [ticker.clone()]}});
                        let _ = ws.send(Message::Text(unsub.to_string())).await;
                        let _ = ws.send(Message::Text(sub.to_string())).await;
                        *last = 0;
                        continue;
                    }
                    *last = seq;
                }
                
                let _ = log_tx.send(json);
            }
        }
    }
    Ok(())
}