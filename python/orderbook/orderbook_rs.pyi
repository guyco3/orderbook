# python/orderbook/orderbook_rs.pyi

class PyRecorder:
    def __init__(
        self, 
        tickers: list[str], 
        api_key: str, 
        key_path: str, 
        log_dir: str = "./logs", 
        debug: bool = False
    ) -> None: 
        """
        Initializes the High-Speed Rust Recorder.
        
        :param tickers: List of Kalshi market tickers (e.g., ["KXNBA..."])
        :param api_key: Your Kalshi API Key ID
        :param key_path: Path to your kalshi_key.pem
        :param log_dir: Where to save JSONL files (default: ./logs)
        :param debug: If True, prints raw WebSocket messages (default: False)
        """
        ...

    def start(self) -> None:
        """Starts the background recording threads."""
        ...