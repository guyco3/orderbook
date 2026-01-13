use base64::{Engine, engine::general_purpose::STANDARD};
use rsa::{
    RsaPrivateKey,
    pkcs1::DecodeRsaPrivateKey,
    pss::SigningKey,
    signature::{RandomizedSigner, SignatureEncoding},
};
use sha2::Sha256;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct KalshiSigner {
    signing_key: SigningKey<Sha256>,
    api_key_id: String,
}

impl KalshiSigner {
    pub fn new(key_path: &str, api_key_id: String) -> Self {
        let path = Path::new(key_path);
        let pem = std::fs::read_to_string(path).unwrap_or_else(|e| {
            let cur_dir = std::env::current_dir().unwrap_or_default();
            panic!(
                "\n❌ FILE NOT FOUND\nLooking for: {:?}\nInside: {:?}\nError: {}\n",
                path, cur_dir, e
            )
        });

        let private_key = RsaPrivateKey::from_pkcs1_pem(&pem)
            .expect("❌ INVALID FORMAT: Key is not PKCS#1. Header must be 'BEGIN RSA PRIVATE KEY'");

        Self {
            signing_key: SigningKey::<Sha256>::new(private_key),
            api_key_id,
        }
    }

    pub fn get_auth_headers(&self) -> (String, String, String) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let msg = format!("{}GET/trade-api/ws/v2", ts);
        let mut rng = rand::thread_rng();
        let signature = self.signing_key.sign_with_rng(&mut rng, msg.as_bytes());
        (
            self.api_key_id.clone(),
            STANDARD.encode(signature.to_bytes()),
            ts,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    // Fix: Import LineEnding directly from the rsa::pkcs1 module
    use rsa::pkcs1::LineEnding;

    #[test]
    fn test_auth_header_generation() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("test_key.pem");

        // Generate a real temporary key for the test
        let mut rng = rand::thread_rng();
        let private_key = rsa::RsaPrivateKey::new(&mut rng, 2048).unwrap();

        // Fix: Use the correct LineEnding reference
        let pem =
            rsa::pkcs1::EncodeRsaPrivateKey::to_pkcs1_pem(&private_key, LineEnding::LF).unwrap();

        let mut file = File::create(&key_path).unwrap();
        file.write_all(pem.as_bytes()).unwrap();

        let signer = KalshiSigner::new(key_path.to_str().unwrap(), "test_id".to_string());
        let (id, sig, ts) = signer.get_auth_headers();

        assert_eq!(id, "test_id");
        assert!(!sig.is_empty());

        // Fix: Changed u140 to u64 (Standard 64-bit unsigned integer)
        assert!(ts.parse::<u64>().is_ok());
    }
}
