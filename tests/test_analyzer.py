import pytest
import pandas as pd
from orderbook.analyzer import Analyzer

def test_orderbook_reconstruction(tmp_path):
    log_dir = tmp_path / "logs"
    log_dir.mkdir()
    log_file = log_dir / "MOCK_TICKER.jsonl"
    
    with open(log_file, "w") as f:
        f.write('{"msg":{"market_ticker":"MOCK","yes":[[93, 1000]],"no":[[5, 500]]},"seq":1,"type":"orderbook_snapshot"}\n')
        f.write('{"msg":{"market_ticker":"MOCK","price":93,"delta":500,"side":"yes","ts":"2026-01-13T00:00:00Z"},"seq":2,"type":"orderbook_delta"}\n')

    ana = Analyzer(log_dir=str(log_dir))
    ana.load_all()
    
    # --- DEBUG SECTION ---
    # If the test fails, this will show us what tables DuckDB actually has
    print("\n🔎 Existing Tables in DuckDB:")
    print(ana.query("SHOW TABLES"))
    # ---------------------

    query = "SELECT current_qty FROM orderbook_mock_ticker WHERE price = 93 AND seq = 2"
    df = ana.query(query)
    
    assert not df.empty, "Query returned no results!"
    assert df['current_qty'].iloc[0] == 1500
    print("✅ Stateful Orderbook Reconstruction Verified!")