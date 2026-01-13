import orderbook
import pandas as pd
import numpy as np

class OrderBookState:
    def __init__(self):
        self.bids = {} # Yes side
        self.asks = {} # No side
    
    def update(self, type, msg):
        if type == 'orderbook_snapshot':
            self.bids = {float(p): float(v) for p, v in msg.get('yes', [])}
            self.asks = {float(p): float(v) for p, v in msg.get('no', [])}
        elif type == 'orderbook_delta':
            price, delta, side = float(msg['price']), float(msg['delta']), msg['side']
            target = self.bids if side == 'yes' else self.asks
            target[price] = target.get(price, 0) + delta
            if target[price] <= 0: target.pop(price, None)

    def get_metrics(self):
        if not self.bids or not self.asks: return None
        best_bid = max(self.bids.keys())
        best_ask = 100 - min(self.asks.keys()) # Normalized to Yes price
        
        b_vol = self.bids[best_bid]
        a_vol = self.asks[min(self.asks.keys())]
        
        # 1. Book Imbalance: Which side is 'heavier'?
        imbalance = (b_vol - a_vol) / (b_vol + a_vol)
        
        # 2. Micro-Price: The 'Fair Value' weighted by volume
        # If the bid is 93 with 1,000,000 contracts and ask is 94 with 10,
        # the micro-price will be 93.99.
        micro_price = (best_bid * a_vol + best_ask * b_vol) / (b_vol + a_vol)
        
        return {
            'mid': (best_bid + best_ask) / 2,
            'micro': micro_price,
            'imbalance': imbalance,
            'spread': best_ask - best_bid,
            'total_liquidity': b_vol + a_vol
        }

def main():
    ana = orderbook.Analyzer(log_dir="./logs")
    ana.load_all()
    # Replace with your actual table name from logs
    ticker_table = "KXFEDDECISION_26JAN_H0" 

    events = ana.query(f"SELECT type, msg FROM {ticker_table} ORDER BY seq ASC")
    book = OrderBookState()
    history = []

    for _, row in events.iterrows():
        book.update(row['type'], row['msg'])
        metrics = book.get_metrics()
        if metrics:
            metrics['type'] = row['type']
            history.append(metrics)

    df = pd.DataFrame(history)

    # 📈 CALCULATE THE ALPHA (Predictive Power)
    # We want to see if 'Imbalance' predicts the Mid-Price move 10 steps later
    df['future_mid'] = df['mid'].shift(-10)
    df['price_move'] = (df['future_mid'] - df['mid'])
    
    # Random Variable: Order Flow Toxicity
    # High positive = market is about to jump. High negative = about to crash.

    df['micro_diff'] = df['micro'].diff().shift(-1)
    correlation = df['imbalance'].corr(df['micro_diff'])

    print("\n" + "="*50)
    print(f"📊 LEVEL 2 QUANT DERIVATION: {ticker_table}")
    print("="*50)
    print(df[['mid', 'micro', 'imbalance', 'spread']].tail(10).to_string(index=False))
    print("-" * 50)
    print(f"🎯 Alpha Correlation (Imbalance -> Price Move): {correlation:.4f}")
    print("="*50)

if __name__ == "__main__":
    main()