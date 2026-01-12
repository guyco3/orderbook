use crate::error::Result;
use std::path::PathBuf;
use std::sync::Arc;
use crate::ingestor::auth::KalshiSigner;

pub mod auth;
pub mod ingestor;
pub mod logger;

pub struct Recorder {
    tickers: Vec<String>,
    log_dir: PathBuf,
    signer: Arc<KalshiSigner>,
    debug: bool,
}

pub struct RecorderBuilder {
    tickers: Vec<String>,
    log_dir: PathBuf,
    api_key_id: Option<String>,
    private_key_path: Option<String>,
    debug: bool,
}

impl RecorderBuilder {
    pub fn new() -> Self {
        Self {
            tickers: vec![],
            log_dir: PathBuf::from("./logs"),
            api_key_id: None,
            private_key_path: None,
            debug: false,
        }
    }

    pub fn with_tickers<S: Into<String>>(mut self, tickers: Vec<S>) -> Self {
        self.tickers = tickers.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn with_log_dir(mut self, path: &str) -> Self {
        self.log_dir = PathBuf::from(path);
        self
    }

    pub fn with_auth(mut self, key_id: String, key_path: String) -> Self {
        self.api_key_id = Some(key_id);
        self.private_key_path = Some(key_path);
        self
    }

    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    pub fn build(self) -> Recorder {
        let key_id = self.api_key_id.expect("API Key ID missing");
        let key_path = self.private_key_path.expect("Private key path missing");
        
        Recorder {
            tickers: self.tickers,
            log_dir: self.log_dir,
            signer: Arc::new(KalshiSigner::new(&key_path, key_id)),
            debug: self.debug,
        }
    }
}

impl Recorder {
    pub fn builder() -> RecorderBuilder { RecorderBuilder::new() }

    pub fn start(&self) -> tokio::task::JoinHandle<Result<()>> {
        let (log_tx, log_rx) = crossbeam_channel::bounded(250_000);
        let log_dir = self.log_dir.clone();

        std::thread::spawn(move || logger::run(log_rx, log_dir));

        let tickers = self.tickers.clone();
        let signer = Arc::clone(&self.signer);
        let debug = self.debug;

        tokio::spawn(async move {
            let mut set = tokio::task::JoinSet::new();
            for chunk in tickers.chunks(20) {
                let tx = log_tx.clone();
                let sig = Arc::clone(&signer);
                let batch = chunk.to_vec();
                set.spawn(async move {
                    // Map generic errors into our custom OrderbookError
                    ingestor::run(tx, batch, sig, debug).await
                        .map_err(|e| crate::error::OrderbookError::Internal(e.to_string()))
                });
            }
            
            while let Some(res) = set.join_next().await {
                // Now these ? work perfectly because the types match
                res??; 
            }
            Ok(())
        })
    }
}