use sha2::{Sha256, Digest};
use crate::models::TokenMetadata;

pub struct VortcoinFramework;

impl VortcoinFramework {
    pub fn tokenize_asset(name: &str, total_supply: u64, doc_bytes: &[u8]) -> TokenMetadata {
        let mut hasher = Sha256::new();
        hasher.update(doc_bytes);
        let doc_hash = hex::encode(hasher.finalize());

        let mut id_hasher = Sha256::new();
        id_hasher.update(format!("{}{}", name, doc_hash));
        let token_id = format!("vort_rwa_{}", hex::encode(&id_hasher.finalize()[12..]));

        TokenMetadata {
            token_id,
            ticker: name.to_uppercase().chars().take(4).collect(),
            token_type: String::from("RWA"),
            total_supply,
            legal_document_hash: Some(doc_hash),
        }
    }

    pub fn mint_memecoin(ticker: &str, total_supply: u64) -> TokenMetadata {
        let mut id_hasher = Sha256::new();
        id_hasher.update(format!("{}{}", ticker, total_supply));
        let token_id = format!("vort_meme_{}", hex::encode(&id_hasher.finalize()[12..]));

        TokenMetadata {
            token_id,
            ticker: ticker.to_uppercase(),
            token_type: String::from("MEMECOIN"),
            total_supply,
            legal_document_hash: None,
        }
    }
}
