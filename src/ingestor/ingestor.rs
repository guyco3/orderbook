// Fix 1: Use 'super' because auth.rs is a sibling in the ingestor folder
use super::auth::KalshiSigner; 

use crossbeam_channel::Sender;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, tungstenite::client::IntoClientRequest};

// Fix 2: Explicitly import HeaderValue from the correct sub-crate
use tokio_tungstenite::tungstenite::http::HeaderValue;

pub async fn run(
    log_tx: Sender<Value>, 
    tickers: Vec<String>, 
    signer: Arc<KalshiSigner>, 
    debug: bool
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (key_id, sig, ts) = signer.get_auth_headers();
    let mut req = url::Url::parse("wss://api.elections.kalshi.com/trade-api/ws/v2")?.into_client_request()?;
    
    let h = req.headers_mut();
    
    // Fix 3: Explicitly parse into HeaderValue. 
    // We use .try_into() or explicit parse to satisfy the compiler's type inference.
    h.insert("KALSHI-ACCESS-KEY", HeaderValue::from_str(&key_id)?);
    h.insert("KALSHI-ACCESS-SIGNATURE", HeaderValue::from_str(&sig)?);
    h.insert("KALSHI-ACCESS-TIMESTAMP", HeaderValue::from_str(&ts)?);

    let (mut ws, _) = connect_async(req).await?;

    // ... rest of your code (subscription logic, seq_map, loop) remains the same ...
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
                
                let ticker = match json["msg"]["market_ticker"].as_str() {
                    Some(t) => t.to_string(),
                    None => continue,
                };

                let msg_type = json["type"].as_str().unwrap_or("");
                
                if msg_type == "orderbook_delta" {
                    let seq = json["seq"].as_u64().unwrap_or(0);
                    let last = seq_map.entry(ticker.clone()).or_insert(0);

                    if *last != 0 && seq != *last + 1 {
                        println!("⚠️ GAP on {}: Expected {}, got {}", ticker, *last + 1, seq);
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