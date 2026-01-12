use base64::{engine::general_purpose::STANDARD, Engine};
use rsa::{pkcs1::DecodeRsaPrivateKey, pss::SigningKey, signature::{RandomizedSigner, SignatureEncoding}, RsaPrivateKey};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;

pub struct KalshiSigner {
    signing_key: SigningKey<Sha256>,
    api_key_id: String,
}

impl KalshiSigner {
    pub fn new(key_path: &str, api_key_id: String) -> Self {
        let path = Path::new(key_path);
        let pem = std::fs::read_to_string(path).unwrap_or_else(|e| {
            let cur_dir = std::env::current_dir().unwrap_or_default();
            panic!("\n❌ FILE NOT FOUND\nLooking for: {:?}\nInside: {:?}\nError: {}\n", path, cur_dir, e)
        });
        
        let private_key = RsaPrivateKey::from_pkcs1_pem(&pem)
            .expect("❌ INVALID FORMAT: Key is not PKCS#1. Header must be 'BEGIN RSA PRIVATE KEY'");
            
        Self {
            signing_key: SigningKey::<Sha256>::new(private_key),
            api_key_id,
        }
    }

    pub fn get_auth_headers(&self) -> (String, String, String) {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
        let msg = format!("{}GET/trade-api/ws/v2", ts);
        let mut rng = rand::thread_rng();
        let signature = self.signing_key.sign_with_rng(&mut rng, msg.as_bytes());
        (self.api_key_id.clone(), STANDARD.encode(signature.to_bytes()), ts)
    }
}