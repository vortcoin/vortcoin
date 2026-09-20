use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub address: String,
    pub balance: u64,                        // In Nano-Vort units (9 decimal places)
    pub rwa_holdings: HashMap<String, u64>,  // AssetID -> Number of digital share fractions
    pub meme_holdings: HashMap<String, u64>, // TokenID -> Number of memecoins owned by the user
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenMetadata {
    pub token_id: String,
    pub ticker: String,
    pub token_type: String, // "RWA" or "MEMECOIN"
    pub total_supply: u64,
    pub legal_document_hash: Option<String>, // Only fill this in if the type is RWA
}

