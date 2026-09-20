use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct P2PSecurityManager {
    // IP Address Identification -> Number of nodes connected from that IP (Sybil prevention)
    pub ip_connection_counters: HashMap<IpAddr, u32>,
    // Maps PeerID -> Block time (blacklist) due to misconduct
    pub blacklisted_peers: HashMap<String, u64>,
}

impl P2PSecurityManager {
    pub fn new() -> Self {
        P2PSecurityManager {
            ip_connection_counters: HashMap::new(),
            blacklisted_peers: HashMap::new(),
        }
    }

    /// 1. PoW Handshake Verification: Ensures the new node is not a mass spam bot
    pub fn verify_handshake_pow(peer_id: &str, nonce: u64, target_difficulty: u32) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(peer_id.as_bytes());
        hasher.update(nonce.to_be_bytes());
        let result = hasher.finalize();

        // Checking whether the hash produces a number of leading zeros that matches the difficulty
        let mut zero_count = 0;
        for &byte in result.iter() {
            if byte == 0 {
                zero_count += 8;
            } else {
                zero_count += byte.leading_zeros();
                break;
            }
        }
        zero_count >= target_difficulty
    }

    /// 2. SYBIL IP FILTER: Limits a maximum of 3 nodes from the same IP address
    pub fn accept_new_connection(&mut self, ip_address: IpAddr) -> Result<(), &'static str> {
        let current_count = self.ip_connection_counters.get(&ip_address).cloned().unwrap_or(0);
        
        if current_count >= 3 {
            return Err("Connection Refused: The maximum node limit (3) for this IP subnet has been reached! (Sybil Protection)");
        }

        self.ip_connection_counters.insert(ip_address, current_count + 1);
        Ok(())
    }

    /// 3. Check Peer Blacklist Status
    pub fn is_peer_blacklisted(&self, peer_id: &str) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if let Some(&expiry) = self.blacklisted_peers.get(peer_id) {
            if now < expiry {
                return true; // The node is still under a 3,690 block penalty period
            }
        }
        false
    }

    /// 4. PENALTY / SLASHING TO NAUGHTY NODES
    pub fn blacklist_malicious_peer(&mut self, peer_id: &str) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let penalty_duration = 3690 * 30; // 3,690 blocks multiplied by a target block time of 30 seconds
        self.blacklisted_peers.insert(peer_id.to_string(), now + penalty_duration);
    }
}

