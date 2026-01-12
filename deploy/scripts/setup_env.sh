#!/bin/bash

# Set up environment variables
cat <<EOT >> /etc/orderbook.env
TICKERS_FILE=/data/tickers.txt
LOG_LEVEL=info
EOT

echo "Environment variables set up successfully."