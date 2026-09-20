use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::models::Account;
use serde::Deserialize;

// Primary Anchor Node Identity Address Locked at the Protocol Consensus Level
pub const VORTCOIN_ANCHOR_NODE_LOCK: &str = "vortcoin_q_pure_anchor_node_matrix_secure_lock";

// Cross-Chain Routing for DEX Wrapped Bridge Modules
pub const VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET: &str = "vortcoin_q_cross_chain_wrapped_bridge_outlet";

#[derive(Deserialize)]
struct GenesisConfig {
    allocations: HashMap<String, AllocationData>,
}

#[derive(Deserialize)]
struct AllocationData {
    amount_nano: u64,
}

pub fn generate_genesis_ledger() -> HashMap<String, Account> {
    let mut genesis_state = HashMap::new();

    // 1. Reading the external configuration manifest genesis.json
    let mut file = File::open("genesis.json").expect("Failed to find the genesis.json configuration file!");
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).expect("Failed to read the genesis.json file structure");
    
    let config: GenesisConfig = serde_json::from_str(&json_str).expect("The JSON format in Genesis.json is invalid");

    // 2. Retrieve the allocation value from JSON; if missing/empty, set the default to 0 NANO.
    // This ensures the anchor account remains registered in the Sled DB with an initial balance of 0 VORT (Pure Fair Launch).
    
    let initial_anchor_balance = match config.allocations.get(VORTCOIN_ANCHOR_NODE_LOCK) {
        Some(data) => data.amount_nano,
        None => 0, // Default to 0 Nano-Vort if the "allocations": {} is empty
    };

    // 3. Inject the clean 0-balance Anchor Account into the Genesis State Ledger
    genesis_state.insert(VORTCOIN_ANCHOR_NODE_LOCK.to_string(), Account {
        address: VORTCOIN_ANCHOR_NODE_LOCK.to_string(),
        balance: initial_anchor_balance,
        rwa_holdings: HashMap::new(),
        meme_holdings: HashMap::new(),
    });

    // 4. Initialize initial empty balance slots for the cross-chain exchange bridge outlet
    genesis_state.insert(VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET.to_string(), Account {
        address: VORTCOIN_CROSS_CHAIN_BRIDGE_OUTLET.to_string(),
        balance: 0,
        rwa_holdings: HashMap::new(),
        meme_holdings: HashMap::new(),
    });

    genesis_state
}
