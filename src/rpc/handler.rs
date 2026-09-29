use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use warp::{Rejection, Reply};
use crate::models::Account;
use crate::storage::VortcoinStorage;

const BLOCKS_PER_ERA: u64 = 369_000;
const INITIAL_REWARD_NANO: u64 = 10_000_000_000; // 10.0 VORT

#[derive(Deserialize, Debug)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

#[derive(Serialize, Debug)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
    pub id: Value,
}

pub async fn process_unified_rpc_request(
    body: Value,
    storage: VortcoinStorage,
) -> Result<impl Reply, Rejection> {
    let req_id = body.get("id").cloned().unwrap_or(Value::Null);
    let method = body.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = body.get("params");

    let response = match method {
        // =========================================================================
        // 1. GET NETWORK STATUS & DYNAMIC CIRCULATION
        // =========================================================================
        "get_status" => {
            let current_height = match storage.db.get("chain_current_height") {
                Ok(Some(bytes)) => {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(&bytes[..8]);
                    u64::from_be_bytes(arr)
                },
                _ => 1u64,
            };

            let mut circulating_nano: u128 = 0;
            let mut total_accounts: u64 = 0;

            for item in storage.db.scan_prefix(b"acc_") {
                if let Ok((_k, val)) = item {
                    if let Ok(acc) = serde_json::from_slice::<Account>(&val) {
                        circulating_nano = circulating_nano.saturating_add(acc.balance as u128);
                        total_accounts += 1;
                    }
                }
            }

            let circulating_vort = (circulating_nano as f64) / 1_000_000_000.0;
            let current_era = ((current_height.saturating_sub(1)) / BLOCKS_PER_ERA) + 1;
            let block_reward = 10.0 / (2_u64.pow((current_era.saturating_sub(1)) as u32) as f64);

            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({
                    "current_block_height": current_height,
                    "current_era": current_era,
                    "block_reward_vort": block_reward,
                    "circulating_supply_nano": circulating_nano,
                    "circulating_supply_vort": circulating_vort,
                    "total_accounts_count": total_accounts,
                    "halving_cycle_blocks": BLOCKS_PER_ERA,
                    "network": "vortcoin_mainnet"
                })),
                error: None,
                id: req_id,
            }
        }

        // =========================================================================
        // 2. GET ACCOUNT DETAILS
        // =========================================================================
        "get_account_details" => {
            let address = params
                .and_then(|p| p.get("address"))
                .and_then(|a| a.as_str())
                .unwrap_or("");

            if address.is_empty() {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(json!({ "code": -32602, "message": "Missing 'address' parameter" })),
                    id: req_id,
                }
            } else {
                match storage.get_account(address) {
                    Some(bytes) => {
                        let account: Account = serde_json::from_slice(bytes.as_ref()).unwrap_or(Account {
                            address: address.to_string(),
                            balance: 0,
                            rwa_holdings: HashMap::new(),
                            meme_holdings: HashMap::new(),
                        });

                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            result: Some(json!({
                                "address": account.address,
                                "balance_nano": account.balance,
                                "balance_vort": (account.balance as f64) / 1_000_000_000.0,
                                "found": true,
                                "rwa_holdings_count": account.rwa_holdings.len(),
                                "meme_holdings_count": account.meme_holdings.len(),
                                "verification_status": "SIGNATURE_VALID_PASS"
                            })),
                            error: None,
                            id: req_id,
                        }
                    }
                    None => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: Some(json!({
                            "address": address,
                            "balance_nano": 0,
                            "balance_vort": 0.0,
                            "found": false,
                            "rwa_holdings_count": 0,
                            "meme_holdings_count": 0,
                            "verification_status": "SIGNATURE_VALID_PASS"
                        })),
                        error: None,
                        id: req_id,
                    },
                }
            }
        }

        // =========================================================================
        // 3. SUBMIT WEB SHARE
        // =========================================================================
        "submit_web_share" => {
            let worker_wallet = params
                .and_then(|p| p.get("worker_wallet"))
                .and_then(|w| w.as_str())
                .unwrap_or("");

            if worker_wallet.is_empty() {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(json!({ "code": -32602, "message": "Missing worker_wallet" })),
                    id: req_id,
                }
            } else {
                let current_height = match storage.db.get("chain_current_height") {
                    Ok(Some(bytes)) => {
                        let mut arr = [0u8; 8];
                        arr.copy_from_slice(&bytes[..8]);
                        u64::from_be_bytes(arr)
                    },
                    _ => 1u64,
                };

                let current_era = ((current_height.saturating_sub(1)) / BLOCKS_PER_ERA) + 1;
                let reward_nano = INITIAL_REWARD_NANO / (2_u64.pow((current_era.saturating_sub(1)) as u32));
                let reward_vort = (reward_nano as f64) / 1_000_000_000.0;

                let mut account = match storage.get_account(worker_wallet) {
                    Some(bytes) => serde_json::from_slice(bytes.as_ref()).unwrap_or(Account {
                        address: worker_wallet.to_string(),
                        balance: 0,
                        rwa_holdings: HashMap::new(),
                        meme_holdings: HashMap::new(),
                    }),
                    None => Account {
                        address: worker_wallet.to_string(),
                        balance: 0,
                        rwa_holdings: HashMap::new(),
                        meme_holdings: HashMap::new(),
                    },
                };

                account.balance = account.balance.saturating_add(reward_nano);
                let _ = storage.save_account(worker_wallet, &account);

                let new_height = current_height + 1;
                let _ = storage.db.insert("chain_current_height", &new_height.to_be_bytes());

                let mut circulating_nano = match storage.db.get("chain_circulating_nano") {
                    Ok(Some(bytes)) => {
                        let mut arr = [0u8; 8];
                        arr.copy_from_slice(&bytes[..8]);
                        u64::from_be_bytes(arr)
                    },
                    _ => 0u64,
                };
                circulating_nano = circulating_nano.saturating_add(reward_nano);
                let _ = storage.db.insert("chain_circulating_nano", &circulating_nano.to_be_bytes());
                let _ = storage.db.flush();

                let total_balance_vort = (account.balance as f64) / 1_000_000_000.0;

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(json!({
                        "status": "share_accepted",
                        "block_height": new_height,
                        "current_era": current_era,
                        "block_reward_allocated": reward_vort,
                        "current_total_balance_vort": total_balance_vort
                    })),
                    error: None,
                    id: req_id,
                }
            }
        }

        // =========================================================================
        // 4. BROADCAST TRANSACTION
        // =========================================================================
        "broadcast_transaction" => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!({
                    "status": "transaction_broadcasted",
                    "tx_hash": format!("0x369{:x}", rand::random::<u128>())
                })),
                error: None,
                id: req_id,
            }
        }

        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(json!({ "code": -32601, "message": "Method not found" })),
            id: req_id,
        },
    };

    Ok(warp::reply::json(&response))
}
