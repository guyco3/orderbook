# System Architecture

The Kalshi Orderbook SDK is designed to handle high-frequency prediction market data without dropping frames or blocking the execution of analysis.

### 1. Ingestion Layer (Rust)
- **Async WebSocket Client:** Uses `tokio-tungstenite` to maintain persistent connections to Kalshi.
- **Message Demuxing:** Inbound JSON is parsed into typed structs.
- **The Buffer Pipe:** Messages are pushed into a thread-safe MPMC channel.

### 2. Persistence Layer (Rust)
- **Vectorized Logging:** Uses a high-capacity `BufWriter`. Instead of writing every message individually, it flushes batches to the NVMe, significantly reducing CPU interrupts.
- **JSONL Format:** Data is stored in Line-Delimited JSON, making it "grep-able" and easy for DuckDB to ingest.

### 3. Analysis Layer (DuckDB + Python)
- **Zero-Copy Ingestion:** DuckDB reads the JSONL files directly as virtual tables.
- **Python Wrappers:** Provides a high-level API for calculating market metrics (Weighted Mid, Imbalance, etc.) using SQL.