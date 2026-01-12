use clap::Parser;
use std::{env, fs::read_to_string};
use std::io::{self, Read};
use orderbook_rs::{Recorder, Result};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    tickers_file: Option<String>,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    
    let tickers_raw = if let Some(path) = args.tickers_file {
        read_to_string(path).expect("❌ File not found")
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let tickers: Vec<String> = tickers_raw.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty()).collect();

    let key_id = env::var("KALSHI_KEY_ID")?;
    let key_path = env::var("KALSHI_PRIVATE_KEY_PATH").unwrap_or("kalshi_key.pem".into());

    let handle = Recorder::builder()
        .with_auth(key_id, key_path)
        .with_tickers(tickers)
        .debug(args.debug)
        .build()
        .start();

    println!("🚀 Recorder online.");

    // THE FIX: Explicitly map the error to the non-thread-safe version for main
    handle.await??;
    
    Ok(())
}