use sled::{Db, IVec};
use serde::Serialize;

#[derive(Clone)] 
pub struct VortcoinStorage {
    pub db: Db,
}

impl VortcoinStorage {
    pub fn init(path: &str) -> Self {
        // Attempting to open the database safely without triggering an immediate panic
        match sled::open(path) {
            Ok(database) => VortcoinStorage { db: database },
            Err(_e) => {
                println!("====================================================");
                println!("[VORTCOIN LEDGER NOTICE] DATABASE IS CURRENTLY IN USE!");
                println!("====================================================");
                println!("Message: Your PoAV mining node is currently active");
                println!("In another terminal and has locked the local database.");
                println!("\n SOLUTION TO CHECK BALANCE:");
                println!("   Use the HTTP RPC Gateway query via the command:");
                println!("   -> curl -X POST http://localhost:8545/rpc");
                println!("====================================================");
                // Exit the program gracefully without triggering a stack trace crash
                std::process::exit(0); 
            }
        }
    }

    pub fn save_account<T: Serialize>(&self, address: &str, account_data: &T) -> Result<(), &'static str> {
        let serialized = serde_json::to_vec(account_data)
            .map_err(|_| "Failed to serialize account data")?;
        self.db.insert(address.as_bytes(), serialized)
            .map_err(|_| "Failed to write to disk")?;
        self.db.flush().map_err(|_| "failed to Lock on disk")?;
        Ok(())
    }

    pub fn get_account(&self, address: &str) -> Option<Vec<u8>> {
        match self.db.get(address.as_bytes()) {
            Ok(Some(ivec)) => Some(ivec.to_vec()),
            _ => None,
        }
    }
}

