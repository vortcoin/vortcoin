use libp2p::request_response::{self, cbor};
use serde::{Serialize, Deserialize};

// Secret Message Structure Between VORTCOIN Nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VortcoinMessage {
    pub sender_address: String,
    pub content: String,
    pub timestamp: u64,
}

// Node Response Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VortcoinResponse {
    pub status_code: u32, // 369 symbolizes cosmic success
    pub status_message: String,
}

pub struct VortcoinMessengerBehaviour {
    // Uses a high-speed Request-Response protocol based on CBOR (Binary JSON)
    pub inner: request_response::cbor::Behaviour<VortcoinMessage, VortcoinResponse>,
}

impl VortcoinMessengerBehaviour {
    pub fn new() -> Self {
        // Set a unique protocol marker for the VORTCOIN communication network
        let protocols = vec![(
            request_response::ProtocolName::new("/vortcoin/messenger/1.3.69"),
            request_response::MessageQuality::Fair,
        )];
        let config = request_response::Config::default();
        
        let inner = request_response::cbor::Behaviour::new(protocols, config);
        VortcoinMessengerBehaviour { inner }
    }
}
