# Kalshi Orderbook SDK 🦀🐍

A high-performance market microstructure engine for Kalshi. This SDK combines a Zero-Copy Rust Ingestor for sub-millisecond data capture with a Vectorized Python Analyzer powered by DuckDB for stateful L2 order book reconstruction.

## 🚀 Key Features
- **Event-Sourced Ingestion:** Rust-based async recorder that preserves message sequence integrity and handles WebSocket gaps automatically.
- **Stateful L2 Reconstruction:** Don't just look at snapshots. The SDK replays orderbook_delta events over orderbook_snapshot anchors using DuckDB window functions.
- **Vectorized SQL Engine:** Query gigabytes of JSONL logs using standard SQL. Perform complex time-series analysis without loading entire files into Python memory.
- **Hybrid Architecture:** Get the performance of Rust's concurrency with the ease of a Pythonic research API.

## 🏗️ Architecture
- **Ingestor (Rust):** Connects to Kalshi's V2 WebSocket, signs requests using RSA-SHA256, and pipes raw messages into high-speed BufWriter channels.
- **Storage (JSONL):** Efficient, ticker-partitioned logs.
- **Analyzer (Python/DuckDB):** Performs the "heavy lifting" of stateful reconstruction, mapping deltas to the last known snapshot to provide a "Ground Truth" view of the book at every sequence ID.

## 📦 Installation

### From Source (Recommended for Dev)

```bash
git clone https://github.com/guyco3/orderbook
cd orderbook
# Install in editable mode to link local Python changes
pip install -e .
```

## 🛠️ Usage Guide

### 1. The Recorder (Rust)

The recorder is optimized for long-running capture sessions. It handles sequence gap detection—if a message is missed, it automatically unsubscribes and resubscribes to catch a fresh snapshot.

```bash
# Provide a tickers.txt file and your PEM key path
cargo run -- --tickers-file tickers.txt --key-path kalshi_key.pem --api-key-id <YOUR_ID>
```

### 2. The Analyzer (Python)

The Analyzer turns raw logs into stateful tables.

```python
from orderbook.analyzer import Analyzer

# 1. Initialize and load all ticker logs in the directory
ana = Analyzer(log_dir="./logs")
ana.load_all()

# 2. Reconstruct the 93-cent "Magnet" wall
# The SDK automatically creates a view: orderbook_{ticker}
market = "KXFEDDECISION_26JAN_H0"
query = f"""
    SELECT seq, price, current_qty 
    FROM orderbook_{market} 
    WHERE price = 93 AND side = 'yes'
    ORDER BY seq DESC LIMIT 10
"""
df = ana.query(query)
print(df)
```

## 🔬 Microstructure Insights

### Detecting "Liquidity Gravity"

The SDK allows you to calculate Micro-Price and Churn Ratios. This helps distinguish between real institutional floors (Stable Walls) and bot-driven "Spoofing" (Flickering Walls).

```python
# Calculate the 'Cost to Sweep' 50,000 contracts
avg_price = ana.estimate_fill(market, side='yes', size=50000)
print(f"Average Fill Price for 50k contracts: {avg_price}c")
```

## 🧪 Testing

The project uses a two-tier testing suite to ensure data integrity.

### Python Logic Tests (pytest)

Verifies the SQL window functions correctly reconstruct the order book state.

```bash
# Standard test run
pytest tests/test_analyzer.py

# Run with stdout to see DuckDB catalog state
pytest -s tests/test_analyzer.py
```

### Rust Core Tests (cargo test)

Verifies RSA signature generation, WebSocket message parsing, and thread-safe logging.

```bash
cargo test
```

## 🏗️ Repository Structure

| Folder   | Responsibility                                      |
|----------|----------------------------------------------------|
| `src/`   | Rust Core: WebSocket client, RSA Auth, Sequence Gap Detection. |
| `python/`| Python SDK: Analyzer class, DuckDB integration, SQL Views. |
| `tests/` | Integrity: pytest and mock log generation.         |
| `examples/` | guess whats in here 0: |

## 📜 License

MIT License. See LICENSE for details.