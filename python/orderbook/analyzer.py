import duckdb
import os

class Analyzer:
    def __init__(self, log_dir="./logs"):
        # We use a persistent connection to an in-memory DB
        self.con = duckdb.connect(database=':memory:')
        self.log_dir = log_dir
        self.con.execute("INSTALL json; LOAD json;")

    def load_all(self):
        """Automatically maps every .jsonl file in the log_dir to a view."""
        for file in os.listdir(self.log_dir):
            if file.endswith(".jsonl"):
                ticker = file.replace(".jsonl", "")
                table_name = ticker.replace("-", "_")
                path = os.path.join(self.log_dir, file)
                
                # We create a view so DuckDB reads directly from the JSONL file
                self.con.execute(f"""
                    CREATE OR REPLACE VIEW {table_name} AS 
                    SELECT * FROM read_json_auto('{path}')
                """)
                print(f"✅ Loaded {ticker} into table {table_name}")

    def query(self, sql):
        """Run custom SQL and return a Pandas DataFrame."""
        return self.con.execute(sql).df()

    def get_market_velocity(self, ticker, interval='1 minute'):
        """
        Calculates the number of ticker events per time interval.
        Ticker 'ts' is a Unix timestamp (seconds).
        """
        table = ticker.replace("-", "_")
        return self.con.execute(f"""
            SELECT 
                time_bucket(interval '{interval}', to_timestamp(cast(msg->>'$.ts' as BIGINT))) as bucket,
                count(*) as event_count
            FROM {table}
            WHERE type = 'ticker'
            GROUP BY 1
            ORDER BY 1
        """).df()