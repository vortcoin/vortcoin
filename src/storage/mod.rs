use sled::{Db, IVec};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::models::Account;

#[derive(Clone)] 
pub struct VortcoinStorage {
    pub db: Db,
}

impl VortcoinStorage {
    pub fn init(path: &str) -> Self {
        match sled::open(path) {
            Ok(database) => VortcoinStorage { db: database },
            Err(_e) => {
                println!("====================================================");
                println!("[VORTCOIN LEDGER NOTICE] DATABASE IS CURRENTLY IN USE!");
                println!("====================================================");
                println!("Message: Your PoAV node is active in another process/terminal");
                println!("and lock the local Sled database folder.");
                println!("\nSOLUTION FOR CHECKING THE BALANCE WITHOUT SHUTTING DOWN THE NODE:");
                println!("Use an HTTP RPC query to the running node:");
                println!("-> curl -s -X POST http://localhost:8545/rpc -H \"Content-Type: application/json\" -d '{{\"jsonrpc\":\"2.0\",\"method\":\"get_account_details\",\"params\":{{\"address\":\"YOUR_WALLET\"}},\"id\":1}}'");
                println!("====================================================");
                std::process::exit(0); 
            }
        }
    }

    /// Save account data to Sled DB
    pub fn save_account<T: Serialize>(&self, address: &str, account_data: &T) -> Result<(), &'static str> {
        let serialized = serde_json::to_vec(account_data)
            .map_err(|_| "Failed to serialize account data")?;
        self.db.insert(address.as_bytes(), serialized)
            .map_err(|_| "Failed to write to disk")?;
        self.db.flush().map_err(|_| "Failed to flush on disk")?;
        Ok(())
    }

    /// Retrieve raw account data from Sled DB
    pub fn get_account(&self, address: &str) -> Option<Vec<u8>> {
        match self.db.get(address.as_bytes()) {
            Ok(Some(ivec)) => Some(ivec.to_vec()),
            _ => None,
        }
    }

    ///Retrieve the Account struct or create a new one if the address does not yet exist in the ledger (Initial Balance: 0)
    pub fn get_or_create_account(&self, address: &str) -> Account {
        if let Some(bytes) = self.get_account(address) {
            if let Ok(acc) = serde_json::from_slice::<Account>(&bytes) {
                return acc;
            }
        }
        
        // New account, if not already in Sled DB
        Account {
            address: address.to_string(),
            balance: 0,
            rwa_holdings: HashMap::new(),
            meme_holdings: HashMap::new(),
        }
    }

    /// Account balance credit (used by PoAV Mining Reward)
    pub fn credit_balance(&self, address: &str, amount_nano: u64) -> Result<u64, &'static str> {
        let mut account = self.get_or_create_account(address);
        account.balance = account.balance.saturating_add(amount_nano);
        self.save_account(address, &account)?;
        Ok(account.balance)
    }

    /// Balance transfer between accounts (used by broadcast_transaction)
    pub fn transfer_balance(
        &self, 
        from_address: &str, 
        to_address: &str, 
        amount_nano: u64, 
        gas_fee_nano: u64
    ) -> Result<(u64, u64), String> {
        let total_deduction = amount_nano.checked_add(gas_fee_nano)
            .ok_or_else(|| "Overflow calculating total transfer deduction".to_string())?;

        let mut sender = self.get_or_create_account(from_address);
        
        if sender.balance < total_deduction {
            return Err(format!(
                "Insufficient balance. Required: {} nanoVORT (including gas), Available: {} nanoVORT", 
                total_deduction, sender.balance
            ));
        }

        let mut receiver = self.get_or_create_account(to_address);

        // Deduct the sender's balance
        sender.balance -= total_deduction;
        // Top up recipient's balance (gas fee burned)
        receiver.balance = receiver.balance.saturating_add(amount_nano);

        self.save_account(from_address, &sender)
            .map_err(|e| format!("Failed to update sender: {}", e))?;
        self.save_account(to_address, &receiver)
            .map_err(|e| format!("Failed to update receiver: {}", e))?;

        Ok((sender.balance, receiver.balance))
    }
}