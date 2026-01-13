import orderbook
import time
import os
from dotenv import load_dotenv
import signal

# 1. Load your Kalshi Keys from .env
load_dotenv()
key_id = os.getenv("KALSHI_KEY_ID")
key_path = os.getenv("KALSHI_PRIVATE_KEY_PATH", "kalshi_key.pem")

# 2. Setup the Rust Recorder
# This spawns the high-speed Rust thread in the background
rec = orderbook.PyRecorder(
    tickers=[
        "KXBTC-26JAN1617-B90750",
        "KXBTC-26JAN1617-B98250",
        "KXBTC-26JAN1617-B99250",
        "KXBTC-26JAN1317-B99750",
        "KXBTC-26JAN1317-B98750",
        ],
    api_key=key_id,
    key_path=key_path,
    log_dir="./logs",
    debug=True
)

print("🏀 Starting Rust Recorder...")
rec.start()

print("🚀 Recorder is live. Press Ctrl+C to exit.")
# This blocks the main thread until a SIGINT (Ctrl+C) is received
signal.sigwait([signal.SIGINT])