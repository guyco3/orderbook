#!/bin/bash

# Install DuckDB
curl -L https://github.com/duckdb/duckdb/releases/latest/download/duckdb_cli-linux-amd64.zip -o duckdb_cli.zip
unzip duckdb_cli.zip -d /usr/local/bin/
chmod +x /usr/local/bin/duckdb

echo "DuckDB installed successfully."