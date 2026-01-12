use crossbeam_channel::Receiver;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn run(rx: Receiver<Value>, log_dir: PathBuf) {
    create_dir_all(&log_dir).ok();
    let mut writers = HashMap::new();

    while let Ok(log) = rx.recv() {
        let ticker = log["ticker"].as_str()
            .or_else(|| log["msg"]["market_ticker"].as_str())
            .unwrap_or("unknown");

        let writer = writers.entry(ticker.to_string()).or_insert_with(|| {
            let file_path = log_dir.join(format!("{}.jsonl", ticker));
            let file = OpenOptions::new().create(true).append(true)
                .open(file_path).unwrap();
            BufWriter::with_capacity(64 * 1024, file)
        });

        if let Ok(line) = serde_json::to_string(&log) {
            let _ = writeln!(writer, "{}", line);
        }

        if rx.is_empty() {
            for w in writers.values_mut() { 
                let _ = w.flush(); 
            }
        }
    }
}