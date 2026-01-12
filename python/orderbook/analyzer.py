import duckdb
import os

class Analyzer:
    def __init__(self, log_dir="./logs"):
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
                
                self.con.execute(f"""
                    CREATE OR REPLACE VIEW {table_name} AS 
                    SELECT * FROM read_json_auto('{path}')
                """)
                print(f"✅ Loaded {ticker} into table {table_name}")

    def query(self, sql):
        """Run custom SQL and return a Pandas DataFrame."""
        return self.con.execute(sql).df()