# examples/heatmap.py
import orderbook
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors

def main():
    ana = orderbook.Analyzer(log_dir="./logs")
    ana.load_all()
    
    ticker = "KXBTC-26JAN1617-B90750"
    table = ticker.replace("-", "_")

    print(f"🌡️ Reconstructing Liquidity Heatmap for {ticker}...")

    # 1. Extract Snapshots and Deltas
    # We ignore Tickers here because we want the RAW book depth
    sql = f"""
        SELECT 
            seq,
            type,
            (msg->>'$.ts')::TIMESTAMP as ts,
            msg as payload
        FROM {table}
        WHERE type IN ('orderbook_snapshot', 'orderbook_delta')
        ORDER BY seq ASC
    """
    df = ana.query(sql)

    # 2. Reconstruct the Book State
    # We'll map price (0-100) vs Time
    time_bins = pd.date_range(start=df['ts'].min(), end=df['ts'].max(), freq='1s')
    heatmap_data = np.zeros((101, len(time_bins)))
    
    current_book = {p: 0 for p in range(101)}

    # Replay logic
    bin_idx = 0
    for _, row in df.iterrows():
        # Update state
        msg = row['payload']
        if row['type'] == 'orderbook_snapshot':
            # Reset book from snapshot
            for level in msg.get('yes', []):
                current_book[level[0]] = level[1]
        else:
            # Apply delta
            price = msg.get('price')
            if price is not None:
                current_book[price] += msg.get('delta', 0)

        # Record state into the current time bin
        while bin_idx < len(time_bins) and row['ts'] >= time_bins[bin_idx]:
            for p in range(101):
                heatmap_data[p, bin_idx] = max(0, current_book[p])
            bin_idx += 1

    # 3. Plotting the Heatmap
    plt.figure(figsize=(15, 8))
    
    # We use a log scale because volume can be 10 or 10,000
    norm = mcolors.LogNorm(vmin=1, vmax=heatmap_data.max())
    
    plt.pcolormesh(time_bins, np.arange(101), heatmap_data, 
                   norm=norm, cmap='magma', shading='auto')
    
    plt.colorbar(label='Number of Contracts (Log Scale)')
    plt.title(f'Orderbook Liquidity Heatmap: {ticker}', fontsize=16)
    plt.ylabel('Price (¢)', fontsize=12)
    plt.xlabel('Time (UTC)', fontsize=12)
    plt.ylim(0, 100) # Binary markets are 0-100
    
    plt.savefig("liquidity_heatmap.png")
    print("🔥 Heatmap saved to liquidity_heatmap.png")

if __name__ == "__main__":
    main()