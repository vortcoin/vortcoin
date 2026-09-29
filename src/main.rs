#![allow(dead_code)]
#![allow(unused_imports)]

use clap::{Parser, Subcommand};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use std::io::Write;
use std::time::{Instant, Duration};
use warp::Filter;
use wasmi::{Engine, Module, Store, Linker, Instance};

pub mod consensus {
    pub mod genesis;
    pub mod price_discovery;
    pub mod poav;
}
pub mod rpc {
    pub mod handler;
}

pub mod models;
pub mod crypto;
pub mod storage;
pub mod rwa;

use models::Account;
use crypto::wallet_secure::SecureVortcoinWallet;
use storage::VortcoinStorage;
use rwa::framework::VortcoinFramework;

mod p2p {
    pub mod security;
}

// =========================================================================
// VORTCOIN COSMIC MACRO CONSTANTS (Tesla 3-6-9 Alignment)
// =========================================================================
pub const MAX_SUPPLY_NANO: u64 = 36_900_000 * 1_000_000_000; // 36.9 Million Pure VORT
pub const BLOCKS_PER_ERA: u64 = 369_000;                     // 369.000 Blok per Era (10% per era)

#[derive(Parser)]
#[command(name = "vortcoin-cli")]
#[command(about = "VORTCOIN CLI - Hard-Sound Scarcity x Monolithic Speed x RWA Assetization Platform", version = "1.3.69")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 1. Create Wallet (Creating a new 24-word BIP-39 quantum-resistant wallet)
    WalletCreate,
    /// 2. Check Balance (Checking balances of the native VORT coin, RWA assets, and memecoins)
    Balance {
        #[arg(short, long)]
        address: String,
    },
    /// 3. VORT Transfer (Send native coin with a 36.9% deflationary burn tax)
    Transfer {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: u64,
    },
    /// 4. Dev-Framework: Mint New Token (Developer Option to Create RWA / Memecoin)
    DevMint {
        #[arg(short, long)]
        ticker: String,
        #[arg(short, long)]
        supply: u64,
        #[arg(short, long)]
        is_rwa: bool,
    },
    /// 5. Market-Metrics: Reviewing local and global market viability statistics
    MarketMetrics,
    /// 6. RWA-Yield: Claim monthly dividends from real-world asset holdings
    YieldClaim {
        #[arg(short, long)]
        address: String,
        #[arg(short, long)]
        asset_id: String,
    },
    /// 7. Node-Start: Activating the Proof of Adaptive Velocity (PoAV) Mining Engine
    NodeStart {
        #[arg(short, long)]
        miner_address: Option<String>,
    },
    /// 8. Bridge-To-Wrapped: Locking native L1 coins to mint wVORT on EVM/Solana
    BridgeToWrapped {
        #[arg(short, long)]
        from_address: String,
        #[arg(short, long)]
        target_network: String,
        #[arg(short, long)]
        target_wallet_escrow: String,
        #[arg(short, long)]
        amount: u64,
    },
    /// 9. Bridge-To-Native: Swap back external wVORT to native L1 VORT coins
    BridgeToNative {
        #[arg(short, long)]
        to_address: String,
        #[arg(short, long)]
        from_network: String,
        #[arg(short, long)]
        proof_tx_hash: String,
        #[arg(short, long)]
        amount: u64,
    },
    /// 10. Core-Import-Snapshot: Emergency migration command
    CoreImportSnapshot {
        #[arg(short, long)]
        file_path: String,
    },
}

/// Bit-Shift Halving Emission Calculation & Global Circulation Upper-Cap Protection
pub fn calculate_adaptive_block_reward(current_height: u64, current_circulating_nano: u64) -> u64 {
    if current_height == 0 {
        return 0;
    }

    // Era 1 spans from Block #1 to #369,000 (10 VORT)
    // Era 2 spans from #369,001 to #738,000 (5 VORT), etc
    let era = (current_height - 1) / BLOCKS_PER_ERA;
    let initial_reward_nano = 10 * 1_000_000_000; // Initial Reward 10.0 VORT

    // Right bit-shift for automatic halving: 10 -> 5 -> 2.5 -> 1.25 ...
    let standard_reward = if era >= 64 { 0 } else { initial_reward_nano >> era };

    // Sacred limit protection: Max Supply of 36.9 million.
    if current_circulating_nano >= MAX_SUPPLY_NANO {
        0 
    } else if current_circulating_nano + standard_reward > MAX_SUPPLY_NANO {
        MAX_SUPPLY_NANO - current_circulating_nano
    } else {
        standard_reward
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let storage = VortcoinStorage::init("./vortcoin_ledger_db");
    
    // INIT GENESIS CHECK: Global system initialization marker
    if storage.db.get("system_genesis_state").unwrap().is_none() {
        println!("[GENESIS LOGIC] Activating Era 0... Reading Public Liquidity Allocations.");
        let genesis_ledger = consensus::genesis::generate_genesis_ledger();
        for (addr, account) in genesis_ledger {
            let _ = storage.save_account(&addr, &account);
        }
        let _ = storage.db.insert("system_genesis_state", b"initialized");
        let _ = storage.db.insert("chain_current_height", &1u64.to_be_bytes());
        let _ = storage.db.insert("chain_circulating_nano", &0u64.to_be_bytes());
        let _ = storage.db.flush();
        println!("Genesis Anchor Vault successfully initialized. Ready for DEX routing.");
    }

    match &cli.command {     
        Commands::WalletCreate => {
            let wallet = SecureVortcoinWallet::create_new_wallet();
            let mut hasher = Sha256::new();
            hasher.update(&wallet.quantum_seed);
            let address_hash = hasher.finalize();
            let vortcoin_address = format!("vort_q_{}", hex::encode(&address_hash[12..]));

            let new_account = Account {
                address: vortcoin_address.clone(),
                balance: 0, 
                rwa_holdings: HashMap::new(),
                meme_holdings: HashMap::new(),
            };
            let _ = storage.save_account(&vortcoin_address, &new_account);

            println!("QUANTUM MULTI-TOKEN WALLET SUCCESSFULLY CREATED!");
            println!("VORTCOIN Public Address   : {}", vortcoin_address);
            println!("Seed Phrase (24 Words): {}\n", wallet.mnemonic_phrase);
        }

        Commands::Balance { address } => {
            if let Some(data) = storage.get_account(address) {
                let account: Account = serde_json::from_slice(&data).unwrap();
                println!("VORTCOIN ON-CHAIN ACCOUNT BALANCE REPORT:");
                println!("----------------------------------------------------");
                println!("Native VORT Balance   : {:.9} VORT", account.balance as f64 / 1_000_000_000.0);
                println!("Real World Assets    : {} Category", account.rwa_holdings.len());
                for (id, amt) in &account.rwa_holdings {
                    println!(" └─ Asset ID [{}] : {} Fractional Shares", id, amt);
                }
                println!("Memecoin Portfolio   : {} Community Tokens", account.meme_holdings.len());
                for (id, amt) in &account.meme_holdings {
                    println!(" └─ Meme ID [{}] : {} Token", id, amt);
                }
                println!("----------------------------------------------------");
            } else {
                println!("Account address not registered in the local node database.");
            }
        }
        
        Commands::Transfer { from, to, amount } => {
            let nano_amount = amount * 1_000_000_000;
            let from_data = storage.get_account(from);
            let to_data = storage.get_account(to);

            if from_data.is_none() || to_data.is_none() {
                println!("Transaction Rejected: Invalid sender or receiver address.");
                return;
            }

            let mut sender: Account = serde_json::from_slice(&from_data.unwrap()).unwrap();
            let mut receiver: Account = serde_json::from_slice(&to_data.unwrap()).unwrap();

            let gas_fee = get_dynamic_gas_fee(&storage); 
            let total_charge = nano_amount + gas_fee;

            if sender.balance < total_charge {
                println!("Transaction Failed: Insufficient balance for transfer + Gas Fee.");
                return;
            }

            let _burned_gas = (gas_fee * 369) / 1000;

            sender.balance -= total_charge;
            receiver.balance += nano_amount;

            let _ = storage.save_account(from, &sender);
            let _ = storage.save_account(to, &receiver);

            println!("TRANSACTION SUCCESSFULLY BROADCASTED TO GLOBAL NETWORK!");
            println!("----------------------------------------------------");
            println!("Amount Transferred  : {} VORT", amount);
            println!("----------------------------------------------------");
        }

        Commands::DevMint { ticker, supply, is_rwa } => {
            println!("[VORTCOIN-SDK FRAMEWORK DEPLOYMENT] Executing token minting process...");
            if *is_rwa {
                let dummy_pdf_sertifikat = format!("SPV Custodian Official Land Certificate for {}", ticker);
                let metadata = VortcoinFramework::tokenize_asset(ticker, *supply, dummy_pdf_sertifikat.as_bytes());
                println!("\n FINANCIAL TOKEN (RWA) SUCCESSFULLY MINTED!");
                println!(" Asset Ticker : {}", metadata.ticker);
                println!(" Token ID Code : {}", metadata.token_id);
                println!(" Document Hash : {}", metadata.legal_document_hash.unwrap());
            } else {
                let metadata = VortcoinFramework::mint_memecoin(ticker, *supply);
                println!("\n COMMUNITY TOKEN (MEMECOIN) SUCCESSFULLY MINTED!");
                println!(" Token Ticker : {}", metadata.ticker);
                println!(" Token ID Code : {}", metadata.token_id);
            }
            println!("----------------------------------------------------");
        }

        Commands::MarketMetrics => {
            let mock_param = consensus::price_discovery::MarketGenesisParam {
                current_network_difficulty: 12,
                estimated_hardware_cost_usd_daily: 0.66,
            };
            let initial_reward_nano = 10 * 1_000_000_000;
            let floor_price = consensus::price_discovery::VortcoinPriceDiscovery::calculate_intrinsic_floor_price(&mock_param, initial_reward_nano);
            let total_daily_emission = 28800.0;
            
            println!("MARKET VIABILITY METRICS (FAIR LAUNCH PHASE):");
            println!("----------------------------------------------------");
            println!(" Active Global Validator Nodes : {} Node", mock_param.current_network_difficulty);
            println!(" Daily New Coin Emission     : {:.0} VORT", total_daily_emission);
            println!(" Estimated Floor Price (VORT)   : ${:.6} USD", floor_price);
            println!("----------------------------------------------------");
        }

        Commands::YieldClaim { address, asset_id } => {
            println!("Connecting to SPV Financial Custodian & Reading Price Oracle...");
            let mock_pool = rwa::yield_engine::RwaYieldPool {
                pool_id: String::from("pool_august_2026"),
                total_accumulated_usd: 10000,
                buyback_rate_vort: 100000,
            };
            match rwa::yield_engine::VortcoinYieldEngine::distribute_rwa_dividend(&storage, address, &mock_pool, asset_id) {
                Ok(reward) => {
                    println!("\n RWA DIVIDEND CLAIM SUCCESSFUL!");
                    println!(" Total VORT credited  : {:.9} VORT", reward as f64 / 1_000_000_000.0);
                },
                Err(e) => println!("\n Claim Failed: {}", e),
            }
        }
        
        Commands::BridgeToWrapped { from_address, target_network, target_wallet_escrow, amount } => {
            let nano_amount = amount * 1_000_000_000;
            let account_data = storage.get_account(from_address);

            if account_data.is_none() {
                println!("Bridge failure: Sender address L1 is not registered.");
                return;
            }

            let mut sender: Account = serde_json::from_slice(&account_data.unwrap()).unwrap();
            let gas_fee = get_dynamic_gas_fee(&storage); 
            let total_charge = nano_amount + gas_fee;

            if sender.balance < total_charge {
                println!("Bridge Failed: Your native VORT balance is insufficient.");
                return;
            }

            let bridge_outlet_data = storage.get_account(consensus::genesis::VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET);
            let mut bridge_outlet: Account = serde_json::from_slice(&bridge_outlet_data.unwrap()).unwrap();

            sender.balance -= total_charge;
            bridge_outlet.balance += nano_amount;

            let _ = storage.save_account(from_address, &sender);
            let _ = storage.save_account(consensus::genesis::VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET, &bridge_outlet);

            println!("=================================================================");
            println!("[VORTCOIN L1 CROSS-CHAIN BRIDGE] ASSET SUCCESSFULLY LOCKED!");
            println!("=================================================================");
            println!("L1 Native Status   : {} VORT successfully locked on-chain.", amount);
            println!("Locking Safe       : {}", consensus::genesis::VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET);
            println!("Target Network     : {}", target_network.to_uppercase());
            println!("Escrow Target      : [{}]", target_wallet_escrow);
            println!("=================================================================");
        }

        Commands::BridgeToNative { to_address, from_network, proof_tx_hash, amount } => {
            let nano_amount = amount * 1_000_000_000;
            let receiver_data = storage.get_account(to_address);

            if receiver_data.is_none() {
                println!("Arbitrage Failed: The destination address is invalid.");
                return;
            }

            if proof_tx_hash.len() < 32 {
                println!("Arbitrage Failed: Tx Hash proof from {} is invalid!", from_network);
                return;
            }

            let bridge_outlet_data = storage.get_account(consensus::genesis::VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET);
            let mut bridge_outlet: Account = serde_json::from_slice(&bridge_outlet_data.unwrap()).unwrap();

            if bridge_outlet.balance < nano_amount {
                println!("CRITICAL: The L1 bridge vault has run out of native liquidity reserves!");
                return;
            }

            let mut receiver: Account = serde_json::from_slice(&receiver_data.unwrap()).unwrap();

            bridge_outlet.balance -= nano_amount;
            receiver.balance += nano_amount;

            let _ = storage.save_account(consensus::genesis::VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET, &bridge_outlet);
            let _ = storage.save_account(to_address, &receiver);

            println!("=================================================================");
            println!("[VORTCOIN L1 ARBITRAGE ENGINE] WRAPPED TO NATIVE LIQUIDITY RELEASED");
            println!("=================================================================");
            println!("Source Network    : {}", from_network.to_uppercase());
            println!("Redemption Result : {} native VORT sent to your L1 wallet.", amount);
            println!("=================================================================");
        }
        
        Commands::CoreImportSnapshot { file_path } => {
            println!("====================================================");
            println!("[VORTCOIN L1 PIVOT ENGINE] STARTING EMERGENCY MIGRATION...");
            println!("====================================================");
            
            let file_res = std::fs::File::open(file_path);
            if file_res.is_err() {
                println!("Migration Failed: Snapshot file '{}' not found.", file_path);
                return;
            }
            
            let mut file = file_res.unwrap();
            let mut json_str = String::new();
            if std::io::Read::read_to_string(&mut file, &mut json_str).is_err() {
                println!("Migration Failed: Snapshot file cannot be read.");
                return;
            }

            let snapshot_data: Result<HashMap<String, u64>, _> = serde_json::from_str(&json_str);
            if snapshot_data.is_err() {
                println!("Migration Failed: The JSON format in the snapshot file is invalid.");
                return;
            }

            let accounts_map = snapshot_data.unwrap();
            let mut imported_count = 0;

            for (address, balance_nano) in accounts_map {
                let synced_account = Account {
                    address: address.clone(),
                    balance: balance_nano,
                    rwa_holdings: HashMap::new(),
                    meme_holdings: HashMap::new(),
                };
                let _ = storage.save_account(&address, &synced_account);
                imported_count += 1;
            }

            println!("Total Accounts Moved: {} Global Addresses", imported_count);
            println!("====================================================");
        }
        
        Commands::NodeStart { miner_address } => {
            println!("====================================================");
            echo_vortcoin_banner();
            println!("Opening P2P Socket Connection on Standard Port: 3690");
            
            let _listener = TcpListener::bind("0.0.0.0:3690");
            
            let target_payout_address = match miner_address.as_ref() {
                Some(addr) => {
                    if addr == consensus::genesis::VORTCOIN_ANCHOR_NODE_LOCK {
                        println!("L1 CONSENSUS ERROR: Node Initialization Rejected! Vault is reserved.");
                        std::process::exit(0);
                    }
                    println!("Active Retail Miner Node: {}", addr);
                    addr.clone()
                },
                None => {
                    println!("NODE ANCHOR DEVELOPER (BOOTNODE) DETECTED!");
                    consensus::genesis::VORTCOIN_ANCHOR_NODE_LOCK.to_string()
                }
            };

            // START RPC GATEWAY ASYNCHRONOUSLY ON PORT 8545
            let storage_rpc = storage.clone();
            tokio::spawn(async move {
                start_rpc_api_gateway(storage_rpc, 8545).await;
            });
            
            //Read the last state from the Sled DB (so it doesn't start from scratch upon restart)
            let mut current_height = match storage.db.get("chain_current_height") {
                Ok(Some(bytes)) => {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(&bytes[..8]);
                    u64::from_be_bytes(arr)
                },
                _ => 1u64,
            };

            let mut current_circulating_nano = match storage.db.get("chain_circulating_nano") {
                Ok(Some(bytes)) => {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(&bytes[..8]);
                    u64::from_be_bytes(arr)
                },
                _ => 0u64,
            };

            let mut current_difficulty: u32 = 12;
            let target_block_time_secs: u64 = 30; // 30-second speed benchmark
            
            // INFINITE MINING LOOP
            loop {
                let block_timer_start = Instant::now();
                let current_era = (current_height - 1) / BLOCKS_PER_ERA + 1;
                let reward_nano = calculate_adaptive_block_reward(current_height, current_circulating_nano);
                
                println!("----------------------------------------------------");
                println!("[Block #{}] Era: {} | Validating Matrix...", current_height, current_era);
                println!("PoAV Adaptive Difficulty: {}", current_difficulty);
                println!("Calculated Block Reward : {:.4} VORT", reward_nano as f64 / 1_000_000_000.0);
                println!("Total Circulating Supply: {:.4} / 36,900,000 VORT", current_circulating_nano as f64 / 1_000_000_000.0);
                
                if reward_nano > 0 {
                    {
                        let account_bytes = storage.get_account(&target_payout_address);
                        let mut miner_account = match account_bytes {
                            Some(bytes) => serde_json::from_slice(&bytes).unwrap(),
                            None => Account {
                                address: target_payout_address.clone(),
                                balance: 0,
                                rwa_holdings: HashMap::new(),
                                meme_holdings: HashMap::new(),
                            },
                        };

                        miner_account.balance = miner_account.balance.saturating_add(reward_nano);
                        let _ = storage.save_account(&target_payout_address, &miner_account);
                        
                        // Accumulate actual circulation
                        current_circulating_nano = current_circulating_nano.saturating_add(reward_nano);
                        let _ = storage.db.insert("chain_circulating_nano", &current_circulating_nano.to_be_bytes());
                        let _ = storage.db.insert("chain_current_height", &(current_height + 1).to_be_bytes());
                        let _ = storage.db.flush();
                        
                        println!("[DB SUCCESS] Ledger Updated. Era {} Block #{} Committed.", current_era, current_height);
                    }

                    let actual_block_time_secs = block_timer_start.elapsed().as_secs();
                    let evaluated_time = if actual_block_time_secs == 0 { 1 } else { actual_block_time_secs };

                    current_difficulty = PoAVConsensus::calculate_adaptive_difficulty(
                        current_difficulty,
                        evaluated_time,
                        target_block_time_secs
                    );
                }

                if reward_nano == 0 && current_height > 1 {
                    println!("[ALERT] Maximum supply cap of 36.9 million reached! Block rewards have ceased permanently.");
                    break;
                }
                
                // Consensus loop heartbeat interval
                tokio::time::sleep(Duration::from_secs(target_block_time_secs.min(5))).await;
                current_height += 1;
            }
            println!("====================================================");
        }        
    }
}

fn echo_vortcoin_banner() {
    println!(" __      __   ____    _____   _______    _____    ____    _____   _   _ ");
    println!(" \\ \\    / /  / __ \\  |  __ \\ |__   __|  / ____|  / __ \\  |_   _| | \\ | |");
    println!("  \\ \\  / /  | |  | | | |__) |   | |    | |      | |  | |   | |   |  \\| |");
    println!("   \\ \\/ /   | |  | | |  _  /    | |    | |      | |  | |   | |   | . ` |");
    println!("    \\  /    | |__| | | | \\ \\    | |    | |____  | |__| |  _| |_  | |\\  |");
    println!("     \\/      \\____/  |_|  \\_\\   |_|     \\_____|  \\____/  |_____| |_| \\_|");
    println!("==========================================================================");
    println!(" VORTCOIN BLOCKCHAIN - PROOF OF ADAPTIVE VELOCITY (PoAV)");
}

// =========================================================================
// RPC API GATEWAY INTERACTION (PRODUCTION-GRADE CORS & CLOUDFLARE COMPATIBLE)
// =========================================================================
async fn start_rpc_api_gateway(storage: storage::VortcoinStorage, port: u16) {
    let storage_filter = warp::any().map(move || storage.clone());

    let cors_policy = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    let rpc_route = warp::post()
        .and(warp::path("rpc"))
        .and(warp::body::json())
        .and(storage_filter)
        .and_then(|body: serde_json::Value, storage_ctx: storage::VortcoinStorage| async move {
            match rpc::handler::process_unified_rpc_request(body, storage_ctx).await {
                Ok(reply) => Ok(warp::reply::with_status(reply, warp::http::StatusCode::OK)),
                Err(rejection) => Err(rejection),
            }
        });
    
    let options_route = warp::options()
        .and(warp::path("rpc"))
        .map(|| warp::reply::with_status(warp::reply(), warp::http::StatusCode::OK)); 

    let complete_gateway_routes = rpc_route
        .or(options_route)
        .with(cors_policy);

    println!("[VORTCOIN-RPC API] Gateway Online at http://0.0.0.0:{}", port);
    println!("Authorized for: Cloudflare Reverse Proxy (rpc.vortcoin.org), Web Miners, and Mobile APK.");

    let host = Ipv4Addr::new(0, 0, 0, 0);
    warp::serve(complete_gateway_routes).run((host, port)).await;
}

// =========================================================================
// DYNAMIC GAS FEE ENGINE
// =========================================================================
fn get_dynamic_gas_fee(storage: &storage::VortcoinStorage) -> u64 {
    let base_gas_fee_nano = 36_900_000; // 0.0369 VORT
    let gas_multiplier = match storage.db.get("network_gas_multiplier") {
        Ok(Some(bytes)) => {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&bytes[..8]);
            u64::from_be_bytes(arr)
        },
        _ => 100,
    };
    (base_gas_fee_nano * gas_multiplier) / 100
}

// =========================================================================
// WASM SMART CONTRACT INTEGRATION MODULE
// =========================================================================

/// Execution of the .wasm-formatted smart contract binary uploaded by the developer
pub fn execute_vortcoin_wasm_contract(wasm_bytecode: &[u8], function_name: &str) -> Result<i32, &'static str> {
    println!("[VORTCOIN-WASM VM] Loading Modular Contract Bytecode for the Second Chain...");
    
    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytecode).map_err(|_| "Corrupt/Invalid WASM Bytecode!")?;
    
    // A place to store state data variables within the smart contract sandbox memory
    let mut store = Store::new(&engine, ());
    let linker = <Linker<()>>::new(&engine);
    
    // Instantiate a modular isolated environment (anti-exploit sandbox)
    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|_| "Failed to instantiate WASM VM Sandbox")?
        .start(&mut store)
        .map_err(|_| "Failed to start Smart Contract runtime engine")?;

    // Calling a specific function (e.g., custom deflationary tax calculation or new RWA rules)
    let func = instance
        .get_typed_func::<(), i32>(&store, function_name)
        .map_err(|_| "Smart Contract function not found in the module")?;

    let result = func.call(&mut store, ()).map_err(|_| "Runtime error during contract execution")?;
    
    println!("[VORTCOIN-WASM VM] Successfully executed function '{}'. Gas used: 0.0369 VORT", function_name);
    Ok(result)
}

// =========================================================================
// PROOF OF ADAPTIVE VELOCITY (PoAV) CONSENSUS ENGINE
// =========================================================================
pub struct PoAVConsensus;

impl PoAVConsensus {
    pub fn calculate_adaptive_difficulty(
        current_difficulty: u32,
        actual_block_time_secs: u64,
        target_block_time_secs: u64
    ) -> u32 {
        let variance_threshold = 5; // 5-second tolerance
        let max_difficulty_cap = 32; 
        let min_difficulty_floor = 12;

        if actual_block_time_secs < (target_block_time_secs - variance_threshold) {
            if current_difficulty < max_difficulty_cap {
                println!("[PoAV INTERCEPTOR] Mining speed too fast ({}s < {}s). Increasing difficulty (+1).", actual_block_time_secs, target_block_time_secs);
                return current_difficulty + 1;
            }
        } else if actual_block_time_secs > (target_block_time_secs + variance_threshold) {
            if current_difficulty > min_difficulty_floor {
                println!("[PoAV INTERCEPTOR] Mining latency detected ({}s > {}s). Reducing difficulty (-1).", actual_block_time_secs, target_block_time_secs);
                return current_difficulty - 1;
            }
        }
        current_difficulty
    }
}