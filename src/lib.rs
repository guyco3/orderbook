use pyo3::prelude::*;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub mod ingestor;
pub mod error;

pub use crate::ingestor::Recorder;
pub use crate::error::{Result, OrderbookError};

// 1. Create a global, static runtime that persists for the life of the Python process
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Failed to create Tokio runtime")
    })
}

#[pyclass]
struct PyRecorder {
    tickers: Vec<String>,
    api_key: String,
    key_path: String,
    log_dir: String,
    debug: bool, // Add this field to the struct
}

#[pymethods]
impl PyRecorder {
    #[new]
    // The signature attribute tells PyO3 how to map Python arguments to Rust
    #[pyo3(signature = (tickers, api_key, key_path, log_dir="./logs".to_string(), debug=false))]
    fn new(
        tickers: Vec<String>, 
        api_key: String, 
        key_path: String, 
        log_dir: String, 
        debug: bool // Match the signature above
    ) -> Self {
        PyRecorder { tickers, api_key, key_path, log_dir, debug }
    }

    fn start(&self) {
        let recorder = Recorder::builder()
            .with_auth(self.api_key.clone(), self.key_path.clone())
            .with_tickers(self.tickers.clone())
            .with_log_dir(&self.log_dir)
            .debug(self.debug) // Pass the debug flag to your internal builder
            .build();
        
        let rt = get_runtime();
        let _guard = rt.enter(); 
        recorder.start();
    }
}

#[pymodule]
fn orderbook_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRecorder>()?;
    Ok(())
}