use duckdb::{params, Connection, Result};

pub struct Analyzer {
    pub conn: Connection,
}

impl Analyzer {
    /// Initialize an in-memory DuckDB instance with JSON support
    pub fn new() -> Result<Self> {
        // Corrected function name below
        let conn = Connection::open_in_memory()?;
        
        // Ensure DuckDB has the JSON extension loaded
        conn.execute_batch("INSTALL json; LOAD json;")?;
        
        Ok(Self { conn })
    }

    /// Scans the logs directory and builds relational views
    pub fn load_logs(&self, logs_dir: &str) -> Result<()> {
        let path = format!("{}/*.jsonl", logs_dir);
        
        let raw_view_query = format!(
            "CREATE OR REPLACE VIEW raw_logs AS 
             SELECT * FROM read_json_auto('{}', union_by_name=true)", 
            path
        );
        self.conn.execute(&raw_view_query, [])?;

        self.conn.execute_batch("
            CREATE OR REPLACE VIEW ticker_data AS
            SELECT 
                msg.market_ticker AS ticker,
                msg.price::DOUBLE AS last_price,
                msg.yes_bid::DOUBLE AS best_bid,
                msg.yes_ask::DOUBLE AS best_ask,
                msg.volume::BIGINT AS volume,
                msg.open_interest::BIGINT AS open_interest,
                to_timestamp(msg.ts::BIGINT) AS ts
            FROM raw_logs 
            WHERE type = 'ticker';

            CREATE OR REPLACE VIEW deltas AS
            SELECT 
                msg.market_ticker AS ticker,
                msg.price::DOUBLE AS price,
                msg.delta::BIGINT AS delta,
                msg.side AS side,
                seq AS sequence,
                msg.ts AS ts_raw,
                CAST(msg.ts AS TIMESTAMP) AS ts
            FROM raw_logs 
            WHERE type = 'orderbook_delta';

            CREATE OR REPLACE VIEW snapshots AS
            SELECT 
                msg.market_ticker AS ticker,
                msg.market_id AS market_id,
                seq AS sequence
            FROM raw_logs 
            WHERE type = 'orderbook_snapshot';
        ")?;

        Ok(())
    }

    pub fn get_total_volume(&self, ticker: &str) -> Result<i64> {
        self.conn.query_row(
            "SELECT MAX(volume) FROM ticker_data WHERE ticker = ?",
            params![ticker],
            |row| row.get(0),
        )
    }
}