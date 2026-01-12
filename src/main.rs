mod auth;
mod ingestor;
mod logger;

use clap::Parser;
use crossbeam_channel::bounded;
use std::{env, fs::read_to_string, sync::Arc};
use tokio::task::JoinSet;
use crate::auth::KalshiSigner;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "tickers.txt")]
    tickers_file: String,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    
    let tickers_raw = read_to_string(&args.tickers_file)
        .expect(&format!("❌ {} not found. Create it with one ticker per line.", args.tickers_file));
    let tickers: Vec<String> = tickers_raw.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    // 250k messages buffer. At 50 connections, this is plenty of headroom for disk lag.
    let (log_tx, log_rx) = bounded(250_000);
    std::thread::spawn(move || logger::run(log_rx));

    let api_key_id = env::var("KALSHI_KEY_ID").expect("KALSHI_KEY_ID not set");
    let key_path = env::var("KALSHI_PRIVATE_KEY_PATH").unwrap_or_else(|_| "kalshi_key.pem".to_string());
    
    println!("🗝️  Using key: {}", key_path);
    let signer = Arc::new(KalshiSigner::new(&key_path, api_key_id));
    
    let mut set = JoinSet::new();
    // Kalshi allows multiple tickers per sub, but 20 is a safe batch for latency
    for chunk in tickers.chunks(20) {
        let tx = log_tx.clone();
        let sig = Arc::clone(&signer);
        let batch = chunk.to_vec();
        let debug = args.debug;
        set.spawn(async move {
            if let Err(e) = ingestor::run(tx, batch, sig, debug).await {
                eprintln!("❌ Ingestor Task failed: {}", e);
            }
        });
    }

    println!("🚀 orderbook online: {} markets. Debug: {}", tickers.len(), args.debug);
    while let Some(_) = set.join_next().await {}
    Ok(())
}