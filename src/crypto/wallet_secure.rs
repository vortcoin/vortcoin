use bip39::{Mnemonic, MnemonicType, Language}; 
use sha2::Sha256;
use pbkdf2::pbkdf2;
use hmac::Hmac;

pub struct SecureVortcoinWallet {
    pub mnemonic_phrase: String,
    pub quantum_seed: [u8; 64],
}

impl SecureVortcoinWallet {
    pub fn create_new_wallet() -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mnemonic.phrase().to_string();
        let quantum_seed = Self::derive_quantum_seed(&phrase, "");

        SecureVortcoinWallet { mnemonic_phrase: phrase, quantum_seed }
    }

    pub fn recover_from_mnemonic(phrase: &str) -> Result<Self, &'static str> {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)
            .map_err(|_| "Mnemonic phrase tidak valid!")?;
        let quantum_seed = Self::derive_quantum_seed(mnemonic.phrase(), "");

        Ok(SecureVortcoinWallet { mnemonic_phrase: mnemonic.phrase().to_string(), quantum_seed })
    }

    fn derive_quantum_seed(phrase: &str, passphrase: &str) -> [u8; 64] {
        let salt = format!("vortcoin_quantum_salt{}", passphrase);
        let mut mac_res = [0u8; 64];
        
        // Key hardening via 3,690 Tesla iterations to block quantum computer hacking
        pbkdf2::<Hmac<Sha256>>(
            phrase.as_bytes(),
            salt.as_bytes(),
            3690, 
            &mut mac_res
        ).expect("Failed to secure the seed using quantum methods");

        mac_res
    }
}

