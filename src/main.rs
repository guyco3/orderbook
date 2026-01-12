use clap::Parser;
use crossbeam_channel::bounded;
use std::{env, fs::read_to_string, sync::Arc};
use tokio::task::JoinSet;
use std::io::{self, Read};

// IMPORT FROM THE LIBRARY ONLY
use orderbook::ingestor::{auth::KalshiSigner, run_ingestor, run_logger};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    tickers_file: Option<String>,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    
    let tickers_raw: String = if let Some(path) = args.tickers_file {
        read_to_string(&path).expect(&format!("❌ {} not found.", path))
    } else {
        if !atty::is(atty::Stream::Stdin) {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        } else {
            eprintln!("❌ Error: No tickers provided. Pipe a file or use --tickers-file");
            std::process::exit(1);
        }
    };

    let tickers: Vec<String> = tickers_raw.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let (log_tx, log_rx) = bounded(250_000);
    
    // Use the alias directly
    std::thread::spawn(move || run_logger(log_rx));

    let api_key_id = env::var("KALSHI_KEY_ID").expect("KALSHI_KEY_ID not set");
    let key_path = env::var("KALSHI_PRIVATE_KEY_PATH").unwrap_or_else(|_| "kalshi_key.pem".to_string());
    
    println!("🗝️  Using key: {}", key_path);
    let signer = Arc::new(KalshiSigner::new(&key_path, api_key_id));
    
    let mut set = JoinSet::new();
    for chunk in tickers.chunks(20) {
        let tx = log_tx.clone();
        let sig = Arc::clone(&signer);
        let batch = chunk.to_vec();
        let debug = args.debug;
        set.spawn(async move {
            // Use the alias directly
            if let Err(e) = run_ingestor(tx, batch, sig, debug).await {
                eprintln!("❌ Ingestor Task failed: {}", e);
            }
        });
    }

    println!("🚀 orderbook online: {} markets. Debug: {}", tickers.len(), args.debug);
    while let Some(_) = set.join_next().await {}
    Ok(())
}