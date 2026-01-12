# examples/analyze.py
import orderbook
import pandas as pd

def main():
    # 1. Initialize the Analyzer
    # It defaults to looking in the "./logs" directory
    ana = orderbook.Analyzer(log_dir="./logs")

    print("🔎 Scanning log directory...")
    
    # 2. Automatically map all .jsonl files to DuckDB views
    # This turns 'logs/KXNBA-LAL-GSW.jsonl' into a table named 'KXNBA_LAL_GSW'
    ana.load_all()

    # 3. Run a custom SQL query
    # Let's look for the 10 most recent 'orderbook_delta' messages 
    # across a specific market.
    ticker_table = "KXNBA_2026_01_12_LAL_GSW" # Adjust this to a ticker you actually recorded
    
    print(f"\n📊 Fetching recent deltas for {ticker_table}...")
    
    # We use DuckDB's JSON extraction syntax: msg->>'$.field'
    query = f"""
        SELECT 
            seq,
            msg->>'$.side' as side,
            (msg->>'$.price')::int as price,
            (msg->>'$.delta')::int as delta
        FROM {ticker_table}
        WHERE type = 'orderbook_delta'
        ORDER BY seq DESC
        LIMIT 10
    """

    try:
        df = ana.query(query)
        
        if df.empty:
            print("Empty result. Have you recorded any data for this ticker yet?")
        else:
            print(df.to_string(index=False))
            
            # 4. Do some quick Pandas analysis
            avg_price = df['price'].mean()
            print(f"\n💡 Average price in last 10 messages: {avg_price:.2f}¢")
            
    except Exception as e:
        print(f"❌ Query failed: {e}")
        print("Tip: Make sure the table name matches your .jsonl filename (with hyphens replaced by underscores).")

if __name__ == "__main__":
    main()