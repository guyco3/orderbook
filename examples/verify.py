from orderbook.analyzer import Analyzer
import matplotlib.pyplot as plt

def main():
    # 1. Setup
    ana = Analyzer(log_dir="./logs")
    ana.load_all()
    
    # Select the Fed Decision market from your logs
    market = "KXFEDDECISION_26JAN_H0"
    
    print(f"📊 Analyzing Reconstructed Orderbook for {market}...")

    # 2. Get the top of the book at every point in time
    # This proves the snapshot + delta reconstruction is working
    df = ana.query(f"""
        SELECT 
            seq,
            MAX(CASE WHEN side = 'yes' THEN price END) as best_bid,
            MAX(CASE WHEN side = 'no' THEN price END) as best_ask,
            SUM(CASE WHEN side = 'yes' THEN current_qty END) as total_bid_depth
        FROM orderbook_{market}
        GROUP BY seq
        ORDER BY seq ASC
    """)

    print(df.tail(10))

    # 3. Plotting the depth evolution
    plt.figure(figsize=(10, 5))
    plt.step(df['seq'], df['total_bid_depth'], where='post', color='green')
    plt.title("L2 Reconstruction: Total Market Depth over Event Sequence")
    plt.xlabel("Event Sequence ID (Deltas)")
    plt.ylabel("Standing Volume (Contracts)")
    plt.savefig("reconstruction_test.png")
    print("✅ Analysis Complete. Check 'reconstruction_test.png' for the depth chart.")

if __name__ == "__main__":
    main()