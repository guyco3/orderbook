# Kalshi Orderbook Ingestor & Analyzer

A high-performance Rust toolset designed to stream real-time order book data from the Kalshi API and provide a professional-grade research environment using DuckDB.

## 🚀 Purpose
This project solves the problem of "Hot Path" data ingestion versus "Cold Path" research. It allows you to maintain stable, low-latency WebSocket connections to hundreds of prediction markets simultaneously while providing a decoupled SQL-based API for backtesting and market microstructure analysis.

## 🏗️ Architecture
The system is split into two distinct domains to ensure that heavy data analysis never interferes with live data collection.

- **The Ingestor (Live):** An asynchronous, multi-threaded worker using tokio and JoinSet. It batches tickers into concurrent WebSocket streams to maximize throughput and minimize sequence gaps.

- **The Logger (Persistence):** A dedicated thread with a 250,000-message buffer that writes raw JSONL files to disk. It uses BufWriter for vectorized SSD writes, optimized for M1/M2 Mac NVMe speeds.

- **The Analysis API (Research):** A library powered by DuckDB. It treats your local log folder as a virtual database, allowing you to run SQL queries across gigabytes of JSON data without a separate database server.

## 🛠️ Setup & Prerequisites

### 1. System Dependencies
You need DuckDB installed on your system for the analysis suite.

```bash
brew install duckdb
```

### 2. Rust Configuration (M1/M2 Macs)
Since DuckDB is installed via Homebrew, you need to tell the Rust linker where to find it. Add this to your `~/.zshrc`:

```bash
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
```

### 3. API Credentials
Create a `.env` file in the project root:

```env
KALSHI_KEY_ID="your-api-key-id"
KALSHI_PRIVATE_KEY_PATH="kalshi_key.pem"
```

## 🖥️ How to Run

### Live Data Ingestion
You can pipe a list of tickers directly into the binary or provide a file path.

#### Using Pipes (Dynamic):
```bash
cat tickers.txt | cargo run --release -- --debug
```

#### Using a File:
```bash
cargo run --release -- --tickers-file tickers.txt
```

### Data Analysis
The analysis tool is built as a library. You can run the built-in integration tests to verify your logged data:

```bash
cargo test --test analysis_test -- --nocapture
```

## 📊 Use Cases & API Features

### Market Microstructure Research
Reconstruct the full Limit Order Book (L2) at any point in history.

```rust
let analyzer = Analyzer::new()?;
analyzer.load_logs("./logs")?;
let book = analyzer.reconstruct_book("KXNFL-26JAN12-HOUPIT")?;
```

### Quant Strategy Backtesting
Calculate spreads, VWAP, and volume imbalance across millions of rows in milliseconds.

```sql
-- Internal DuckDB View Example
SELECT AVG(price) FROM ticker_data WHERE ticker = 'KXNBA...'
```

### Export for Python/R
Clean and compress your raw JSON logs into Parquet files for use in Pandas or Polars.

```rust
analyzer.export_to_parquet("ticker_name", "output.parquet")?;
```

## 📁 Repository Structure
```
.
├── src/
│   ├── main.rs          # Live Ingestor CLI
│   ├── lib.rs           # Core Library Entry
│   ├── ingestor/        # WebSocket & Auth Logic
│   └── analysis/        # DuckDB & Math Logic
├── tests/               # Integration & Research tests
├── logs/                # Auto-generated JSONL data lake
└── tickers.txt          # Default market list
```

## 📜 License
This project is licensed under the [Apache License](LICENSE).