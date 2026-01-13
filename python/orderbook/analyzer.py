import duckdb
import os

class Analyzer:
    def __init__(self, log_dir="./logs"):
        self.con = duckdb.connect(database=":memory:")
        self.log_dir = log_dir
        self.con.execute("INSTALL json; LOAD json;")

    def load_all(self):
        for file in os.listdir(self.log_dir):
            if not file.endswith(".jsonl"): continue
            
            ticker = file.replace(".jsonl", "")
            # 🟢 FIX: Lowercase the table name to make it DuckDB-friendly
            table = ticker.replace("-", "_").lower()
            path = os.path.join(self.log_dir, file)

            # 1️⃣ Load Raw Events
            self.con.execute(f"""
                CREATE OR REPLACE TABLE raw_{table} AS
                SELECT 
                    COALESCE(msg->>'$.market_ticker', msg->'msg'->>'$.market_ticker') as market_ticker,
                    type, CAST(seq AS BIGINT) as seq, msg 
                FROM read_json_auto('{path}')
            """)

            # 2️⃣ Explode Snapshots
            self.con.execute(f"""
                CREATE OR REPLACE TABLE snapshots_{table} AS
                SELECT market_ticker, seq as snap_seq, 'yes' as side, 
                       CAST(level[1] AS INT) as price,
                       CAST(level[2] AS BIGINT) as qty
                FROM (
                    SELECT market_ticker, seq, unnest(CAST(msg->'$.yes' AS DOUBLE[][])) as level
                    FROM raw_{table} WHERE type = 'orderbook_snapshot'
                )
                UNION ALL
                SELECT market_ticker, seq as snap_seq, 'no' as side, 
                       CAST(level[1] AS INT) as price,
                       CAST(level[2] AS BIGINT) as qty
                FROM (
                    SELECT market_ticker, seq, unnest(CAST(msg->'$.no' AS DOUBLE[][])) as level
                    FROM raw_{table} WHERE type = 'orderbook_snapshot'
                )
            """)

            # 3️⃣ Deltas with Epoch Mapping
            self.con.execute(f"""
                CREATE OR REPLACE TABLE deltas_{table} AS
                SELECT 
                    market_ticker, seq, 
                    msg->>'$.side' as side,
                    CAST(msg->'$.price' AS INT) as price,
                    CAST(msg->'$.delta' AS BIGINT) as delta,
                    MAX(CASE WHEN type='orderbook_snapshot' THEN seq END) 
                        OVER (PARTITION BY market_ticker ORDER BY seq) as last_snap_seq
                FROM raw_{table}
                WHERE type IN ('orderbook_delta', 'orderbook_snapshot')
            """)

            # 4️⃣ Stateful Reconstructed Orderbook View
            self.con.execute(f"""
                CREATE OR REPLACE VIEW orderbook_{table} AS
                WITH delta_sums AS (
                    SELECT last_snap_seq, side, price, seq,
                           SUM(delta) OVER (PARTITION BY last_snap_seq, side, price ORDER BY seq) as cum_delta
                    FROM deltas_{table} WHERE delta IS NOT NULL
                )
                SELECT 
                    COALESCE(d.seq, s.snap_seq) as seq,
                    COALESCE(d.side, s.side) as side,
                    COALESCE(d.price, s.price) as price,
                    (COALESCE(s.qty, 0) + COALESCE(d.cum_delta, 0)) as current_qty
                FROM delta_sums d
                FULL OUTER JOIN snapshots_{table} s 
                  ON d.last_snap_seq = s.snap_seq 
                 AND d.side = s.side 
                 AND d.price = s.price
                WHERE current_qty > 0
            """)
            print(f"✅ Reconstructed L2 Book: orderbook_{table}")

    def query(self, sql):
        return self.con.execute(sql).df()