# Understanding Kalshi Market Data

### Orderbook Delta vs Ticker
- **Deltas:** These represent *intent*. A delta of `-100` at 50¢ could be a trade filling an order, or a trader canceling their limit order.
- **Tickers:** These represent *reality*. A ticker update confirms executed volume, open interest changes, and the true "Last Price."

### Time Precision
The SDK captures timestamps with microsecond precision.
1. `ts` in deltas: Server-side event time (ISO 8601).
2. `Clock` in tickers: Sequential engine counter.

### Price Convention
All prices are represented in cents (1-99). 
- `yes_bid`: The highest price someone is willing to pay for 'Yes'.
- `no_bid`: The highest price someone is willing to pay for 'No'.