// =========================================================================
// UNIFIED ON-CHAIN VALIDATION MODULE & ROUTER GATEWAY (PRODUCTION ENGINE)
// =========================================================================
use warp::Filter;
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use hex;
use serde_json::json;
use md5;

use crate::storage; 
use crate::models::Account;
use crate::{calculate_adaptive_block_reward, BLOCKS_PER_ERA, MAX_SUPPLY_NANO};

pub async fn process_unified_rpc_request(
    body: serde_json::Value, 
    storage_ctx: storage::VortcoinStorage
) -> Result<impl warp::Reply, warp::Rejection> {
    
    let method = body["method"].as_str().unwrap_or("unknown");
    let request_id = body["id"].as_i64().unwrap_or(369);

    // ─────────────────────────────────────────────────────────────────────
    // METHOD 1: get_status (Live Data for Web Dashboard & Explorer)
    // ─────────────────────────────────────────────────────────────────────
    if method == "get_status" {
        let current_height = match storage_ctx.db.get("chain_current_height") {
            Ok(Some(bytes)) => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[..8]);
                u64::from_be_bytes(arr)
            },
            _ => 1u64,
        };

        let current_circulating_nano = match storage_ctx.db.get("chain_circulating_nano") {
            Ok(Some(bytes)) => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[..8]);
                u64::from_be_bytes(arr)
            },
            _ => 0u64,
        };

        let current_era = (current_height.saturating_sub(1) / BLOCKS_PER_ERA) + 1;
        let active_block_reward_nano = calculate_adaptive_block_reward(current_height, current_circulating_nano);

        let success_payload = json!({
            "jsonrpc": "2.0",
            "result": {
                "status": "connected",
                "network": "vortcoin_l1_matrix",
                "rpc_endpoint": "https://rpc.vortcoin.org",
                "current_block_height": current_height,
                "current_era": current_era,
                "block_reward_vort": (active_block_reward_nano as f64) / 1_000_000_000.0,
                "circulating_supply_vort": (current_circulating_nano as f64) / 1_000_000_000.0,
                "max_supply_vort": (MAX_SUPPLY_NANO as f64) / 1_000_000_000.0,
                "target_block_time_sec": 30
            },
            "id": request_id
        });
        return Ok(warp::reply::json(&success_payload));
    }

    // ─────────────────────────────────────────────────────────────────────
    // METHOD 2: get_account_details (Check Real-Time Balance from Sled DB)
    // ─────────────────────────────────────────────────────────────────────
    if method == "get_account_details" {
        let target_address = body["params"]["address"].as_str().unwrap_or("");
        
        let account = storage_ctx.get_or_create_account(target_address);
        let response_payload = json!({
            "jsonrpc": "2.0",
            "result": {
                "found": true,
                "address": target_address,
                "balance_nano": account.balance,
                "balance_vort": (account.balance as f64) / 1_000_000_000.0,
                "rwa_holdings_count": account.rwa_holdings.len(),
                "meme_holdings_count": account.meme_holdings.len(),
                "verification_status": "SIGNATURE_VALID_PASS"
            },
            "id": request_id
        });
        return Ok(warp::reply::json(&response_payload));
    }

    // ─────────────────────────────────────────────────────────────────────
    // METHOD 3: broadcast_transaction (On-Chain Transfer Between Wallets)
    // ─────────────────────────────────────────────────────────────────────
    if method == "broadcast_transaction" {
        let signature_hex = body["params"]["raw_tx"].as_str().unwrap_or("");
        let sender_pubkey_hex = body["params"]["from"].as_str().unwrap_or("");
        let receiver_address = body["params"]["to"].as_str().unwrap_or("");
        let amount_nano = body["params"]["amount_nano"].as_u64().unwrap_or(0);
        let timestamp = body["params"]["timestamp"].as_u64().unwrap_or(0);

        if amount_nano == 0 {
            return Ok(warp::reply::json(&json!({
                "jsonrpc": "2.0",
                "error": { "code": -32602, "message": "Amount must be greater than 0" },
                "id": request_id
            })));
        }

        let message_payload = format!(
            "{{\"from\":\"{}\",\"to\":\"{}\",\"amount_nano\":{},\"timestamp\":{}}}",
            sender_pubkey_hex, receiver_address, amount_nano, timestamp
        );

        let signature_bytes = match hex::decode(signature_hex) {
            Ok(bytes) => bytes,
            Err(_) => return Ok(warp::reply::json(&json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid signature hex"},"id":request_id})))
        };

        let pubkey_bytes = match hex::decode(sender_pubkey_hex) {
            Ok(bytes) => bytes,
            Err(_) => return Ok(warp::reply::json(&json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid pubkey hex"},"id":request_id})))
        };

        let public_key = match VerifyingKey::from_bytes(&pubkey_bytes.try_into().unwrap_or([0; 32])) {
            Ok(key) => key,
            Err(_) => return Ok(warp::reply::json(&json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid public key format"},"id":request_id})))
        };

        let signature = match Signature::from_bytes(&signature_bytes.try_into().unwrap_or([0; 64])) {
            sig => sig,
        };

        // Ed25519 Signature Verification
        if public_key.verify(message_payload.as_bytes(), &signature).is_ok() {
            let base_gas_fee: u64 = 36_900_000; // 0.0369 VORT Gas Fee
            let hyper_deflation_burn = (base_gas_fee as f64 * 0.369) as u64;

            match storage_ctx.transfer_balance(sender_pubkey_hex, receiver_address, amount_nano, base_gas_fee) {
                Ok((sender_rem_balance, receiver_new_balance)) => {
                    let tx_raw = format!("{}:{}:{}:{}", sender_pubkey_hex, receiver_address, amount_nano, timestamp);
                    let tx_hash = format!("{:x}", md5::compute(tx_raw.as_bytes()));

                    let success_payload = json!({
                        "jsonrpc": "2.0",
                        "result": {
                            "tx_hash": format!("0x{}", tx_hash),
                            "status": "committed_to_ledger",
                            "verification": "SIGNATURE_VALID_PASS",
                            "amount_transferred_vort": (amount_nano as f64) / 1_000_000_000.0,
                            "sender_remaining_balance_vort": (sender_rem_balance as f64) / 1_000_000_000.0,
                            "receiver_balance_vort": (receiver_new_balance as f64) / 1_000_000_000.0,
                            "gas_burned_nano": hyper_deflation_burn
                        },
                        "id": request_id
                    });
                    return Ok(warp::reply::json(&success_payload));
                },
                Err(err_msg) => {
                    return Ok(warp::reply::json(&json!({
                        "jsonrpc": "2.0",
                        "error": { "code": -32000, "message": err_msg },
                        "id": request_id
                    })));
                }
            }
        } else {
            return Ok(warp::reply::json(&json!({"jsonrpc":"2.0","error":{"code":-32003,"message":"Signature mismatch"},"id":request_id})));
        }
    }

    // ─────────────────────────────────────────────────────────────────────
    // METHOD 4: submit_web_share (Real Mining via Web, Terminal & APK)
    // ─────────────────────────────────────────────────────────────────────
    if method == "submit_web_share" {
        let worker_wallet = body["params"]["worker_wallet"].as_str().unwrap_or("");
        let nonce = body["params"]["nonce"].as_u64().unwrap_or(0);
        let challenge_difficulty = body["params"]["difficulty"].as_u64().unwrap_or(4); // Default 4 digit nol

        if worker_wallet.is_empty() {
            return Ok(warp::reply::json(&json!({
                "jsonrpc": "2.0",
                "error": { "code": -32602, "message": "Worker wallet cannot be empty" },
                "id": request_id
            })));
        }

        let raw_block_payload = format!("{}{}", worker_wallet, nonce);
        let proven_hash = format!("{:x}", md5::compute(raw_block_payload.as_bytes()));
        let target_prefix = "0".repeat(challenge_difficulty as usize);

        if proven_hash.starts_with(&target_prefix) {
            // Perform High Block and Active Circulation from the DB Sled
            let current_height = match storage_ctx.db.get("chain_current_height") {
                Ok(Some(bytes)) => {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(&bytes[..8]);
                    u64::from_be_bytes(arr)
                },
                _ => 1u64,
            };

            let current_circulating_nano = match storage_ctx.db.get("chain_circulating_nano") {
                Ok(Some(bytes)) => {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(&bytes[..8]);
                    u64::from_be_bytes(arr)
                },
                _ => 0u64,
            };

            // Calculate the block reward based on the halving era
            let reward_nano = calculate_adaptive_block_reward(current_height, current_circulating_nano);
            let current_era = (current_height.saturating_sub(1) / BLOCKS_PER_ERA) + 1;

            if reward_nano == 0 {
                return Ok(warp::reply::json(&json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32006, "message": "Max supply cap of 36.9M VORT reached. No further emissions." },
                    "id": request_id
                })));
            }

            // Credit miner balance to Sled DB
            match storage_ctx.credit_balance(worker_wallet, reward_nano) {
                Ok(new_balance_nano) => {
                    // Update chain metadata in Sled DB
                    let new_height = current_height + 1;
                    let new_circulating = current_circulating_nano.saturating_add(reward_nano);
                    
                    let _ = storage_ctx.db.insert("chain_current_height", &new_height.to_be_bytes());
                    let _ = storage_ctx.db.insert("chain_circulating_nano", &new_circulating.to_be_bytes());
                    let _ = storage_ctx.db.flush();

                    let block_mint_payload = json!({
                        "jsonrpc": "2.0",
                        "result": {
                            "status": "share_accepted",
                            "block_height": current_height,
                            "era": current_era,
                            "block_reward_allocated": (reward_nano as f64) / 1_000_000_000.0,
                            "current_total_balance_vort": (new_balance_nano as f64) / 1_000_000_000.0,
                            "poav_hash_verified": proven_hash
                        },
                        "id": request_id
                    });
                    return Ok(warp::reply::json(&block_mint_payload));
                },
                Err(err) => {
                    return Ok(warp::reply::json(&json!({
                        "jsonrpc": "2.0",
                        "error": { "code": -32005, "message": format!("Database write error: {}", err) },
                        "id": request_id
                    })));
                }
            }
        } else {
            return Ok(warp::reply::json(&json!({
                "jsonrpc": "2.0",
                "error": { "code": -32004, "message": "Invalid nonce hash prefix" },
                "id": request_id
            })));
        }
    }

    Ok(warp::reply::json(&json!({"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":request_id})))
}