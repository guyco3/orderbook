use super::engine::Analyzer;
use super::models::{LocalOrderbook, OrderbookLevel};
use duckdb::{params, Result};
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

pub trait OrderbookTools {
    fn reconstruct_book(&self, ticker: &str) -> Result<LocalOrderbook>;
}

impl OrderbookTools for Analyzer {
    fn reconstruct_book(&self, ticker: &str) -> Result<LocalOrderbook> {
        let mut stmt = self.conn.prepare("
            SELECT 
                price, 
                SUM(delta) as total_qty, 
                side 
            FROM deltas 
            WHERE ticker = ? 
            GROUP BY price, side
            HAVING total_qty > 0
            ORDER BY price DESC
        ")?;

        let rows = stmt.query_map(params![ticker], |row| {
            let price: f64 = row.get(0)?;
            let qty: i64 = row.get(1)?;
            let side: String = row.get(2)?;
            Ok((price, qty, side))
        })?;

        let mut bids = BTreeMap::new();
        let mut asks = BTreeMap::new();

        for row in rows {
            let (price, qty, side) = row?;
            if side == "yes" {
                bids.insert(OrderedFloat(price), qty);
            } else {
                asks.insert(OrderedFloat(price), qty);
            }
        }

        Ok(LocalOrderbook {
            ticker: ticker.to_string(),
            bids: bids.iter().rev().map(|(&p, &q)| OrderbookLevel { price: p.into_inner(), quantity: q }).collect(),
            asks: asks.iter().map(|(&p, &q)| OrderbookLevel { price: p.into_inner(), quantity: q }).collect(),
            timestamp: "reconstructed".to_string(),
        })
    }
}