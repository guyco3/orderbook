#!/usr/bin/env bash

API="https://api.elections.kalshi.com/trade-api/v2/markets"
OUT="tickers.txt"

rm -f "$OUT"

cursor=""
page=1

while true; do
  echo "Fetching page $page..."

  if [ -z "$cursor" ]; then
    resp=$(curl -s "$API?limit=1000")
  else
    resp=$(curl -s "$API?limit=1000&cursor=$cursor")
  fi

  # Append tickers
  echo "$resp" | jq -r '.markets[].ticker' >> "$OUT"

  # Read cursor
  cursor=$(echo "$resp" | jq -r '.cursor')

  # Stop if no more pages
  if [ "$cursor" = "null" ] || [ -z "$cursor" ]; then
    break
  fi

  page=$((page + 1))
  sleep 0.2   # be polite to rate limits
done

echo "Done. Total tickers:"
wc -l "$OUT"

