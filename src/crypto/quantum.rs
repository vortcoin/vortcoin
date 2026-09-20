use sha2::{Sha256, Digest};
use rand::rngs::OsRng;
use rand::RngCore;

pub const SIGNATURE_ELEMENTS: usize = 256;

pub struct VortcoinQuantumKeyPair {
    private_key_pairs: Vec<([u8; 32], [u8; 32])>,
    pub public_key_pairs: Vec<([u8; 32], [u8; 32])>,
}

impl VortcoinQuantumKeyPair {
    pub fn generate_from_seed(seed: &[u8; 64]) -> Self {
        let mut priv_pairs = Vec::with_capacity(SIGNATURE_ELEMENTS);
        let mut pub_pairs = Vec::with_capacity(SIGNATURE_ELEMENTS);

        // Deriving keys deterministically from the Seed using chained SHA-256 hashing.
        for i in 0..SIGNATURE_ELEMENTS {
            let mut hasher_0 = Sha256::new();
            hasher_0.update(seed);
            hasher_0.update(format!("vortcoin_q_priv_0_{}", i).as_bytes());
            let mut secret_0 = [0u8; 32];
            secret_0.copy_from_slice(&hasher_0.finalize());

            let mut hasher_1 = Sha256::new();
            hasher_1.update(seed);
            hasher_1.update(format!("vortcoin_q_priv_1_{}", i).as_bytes());
            let mut secret_1 = [0u8; 32];
            secret_1.copy_from_slice(&hasher_1.finalize());

            let hash_0 = Self::hash_leaf(&secret_0);
            let hash_1 = Self::hash_leaf(&secret_1);

            priv_pairs.push((secret_0, secret_1));
            pub_pairs.push((hash_0, hash_1));
        }

        VortcoinQuantumKeyPair { private_key_pairs: priv_pairs, public_key_pairs: pub_pairs }
    }

    fn hash_leaf(data: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let mut result = [0u8; 32];
        result.copy_from_slice(&hasher.finalize());
        result
    }
}
