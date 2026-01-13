# Kalshi Orderbook SDK 🦀🐍

[![PyPI version](https://badge.fury.io/py/kalshi-orderbook.svg)](https://pypi.org/project/kalshi-orderbook/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance market data engine for Kalshi. It uses a **Rust core** for low-latency WebSocket ingestion and **DuckDB** for lightning-fast microstructure analysis.

![Market Liquidity Heatmap](https://raw.githubusercontent.com/guyco3/orderbook/main/liquidity_heatmap.png)

## 🚀 Key Features
- **Zero-Latency Ingestion:** Rust-based async recorder with vectorized SSD writes.
- **DuckDB Integration:** Query gigabytes of JSONL logs directly with SQL—no database setup required.
- **Python-Native:** Install via `pip` and get the performance of Rust with the ease of Python.
- **Microstructure Suite:** Reconstruct L2 order books and visualize market "heatmaps" out of the box.

## 📦 Installation

```bash
pip install kalshi-orderbook
```

Note: Requires Python 3.7+ and a working Rust toolchain for manual builds.

## 🛠️ Quick Start

### 1. Record Live Data (CLI)

You can use the built-in Rust ingestor to record specific tickers:

```bash
# Provide a file with tickers
cargo run --release -- --tickers-file tickers.txt
```

### 2. High-Res Analysis (Python)

Use the SDK to analyze your logs with microsecond precision.

```python
import orderbook

# Load logs from the local directory
ana = orderbook.Analyzer(log_dir="./logs")
ana.load_all()

# Run a SQL query across your JSON data
df = ana.query("SELECT * FROM KXBTC_26JAN_B90750 WHERE type = 'ticker'")
print(df)
```

## 📊 Market Microstructure

The SDK allows you to visualize the "hidden" state of the market, including liquidity walls and bot activity.

- **Velocity Analysis:** Detect micro-bursts of market activity.
- **Heatmap Reconstruction:** Replay the order book to see where depth is concentrating.

## 🏗️ Repository Structure

```
.
├── src/                # High-performance Rust Engine
├── python/             # Python SDK Wrappers
├── examples/           # Ready-to-run analysis scripts
├── logs/               # Local JSONL data lake
├── Cargo.toml          # Rust dependencies
└── pyproject.toml      # Python metadata (Maturin)
```

## 🤝 Contributing

Contributions are welcome! Please see CONTRIBUTING.md for our development standards and how to submit pull requests.

## 📜 License

Licensed under the MIT License.