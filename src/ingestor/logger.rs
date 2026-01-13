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
        let ticker = log["ticker"]
            .as_str()
            .or_else(|| log["msg"]["market_ticker"].as_str())
            .unwrap_or("unknown");

        let writer = writers.entry(ticker.to_string()).or_insert_with(|| {
            let file_path = log_dir.join(format!("{}.jsonl", ticker));
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::thread;
    use tempfile::tempdir;

    #[test]
    fn test_logger_integration_writes_to_disk() {
        // 1. Setup a temporary directory
        let dir = tempdir().unwrap();
        let log_dir = dir.path().to_path_buf();

        // 2. Create the channel and spawn the logger in a background thread
        let (tx, rx) = crossbeam_channel::unbounded();
        let log_dir_clone = log_dir.clone();

        let handle = thread::spawn(move || {
            run(rx, log_dir_clone);
        });

        // 3. Send diverse messages (Kalshi style and Standard style)
        let msg1 = json!({"ticker": "BTC-2026", "price": 93000});
        let msg2 = json!({"msg": {"market_ticker": "FED-JAN"}, "delta": 500});

        tx.send(msg1).unwrap();
        tx.send(msg2).unwrap();

        // 4. Drop the sender to close the channel and allow the loop to finish
        drop(tx);
        handle.join().unwrap(); // Wait for the logger thread to finish writing

        // 5. VERIFY: Did the files actually get created with the right names?
        let file1_path = log_dir.join("BTC-2026.jsonl");
        let file2_path = log_dir.join("FED-JAN.jsonl");

        assert!(file1_path.exists(), "BTC-2026.jsonl was not created");
        assert!(file2_path.exists(), "FED-JAN.jsonl was not created");

        // 6. VERIFY: Is the content inside correct?
        let content1 = fs::read_to_string(file1_path).unwrap();
        assert!(content1.contains("93000"));

        let content2 = fs::read_to_string(file2_path).unwrap();
        assert!(content2.contains("FED-JAN"));
        assert!(content2.contains("500"));
    }
}
