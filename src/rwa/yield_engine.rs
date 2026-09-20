use serde::{Serialize, Deserialize};
use crate::models::Account;
use crate::storage::VortcoinStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RwaYieldPool {
    pub pool_id: String,
    pub total_accumulated_usd: u64,
    pub buyback_rate_vort: u64,
}

pub struct VortcoinYieldEngine;

impl VortcoinYieldEngine {
    /// Distributing RWA profits to digital fractional shareholders on a pro-rata basis
    pub fn distribute_rwa_dividend(
        storage: &VortcoinStorage,
        user_address: &str,
        pool: &RwaYieldPool,
        asset_id: &str
    ) -> Result<u64, &'static str> {
        // Retrieve account data from the local Sled database
        let account_bytes = storage.get_account(user_address)
            .ok_or("The claimant's wallet address was not found in the ledger")?;
            
        let mut account: Account = serde_json::from_slice(&account_bytes)
            .map_err(|_| "Failed to read account data structure")?;

        // Check if the user actually owns the relevant RWA digital sheet
        let user_shares = account.rwa_holdings.get(asset_id)
            .cloned()
            .unwrap_or(0);

        if user_shares == 0 {
            return Err("Rejected: You do not hold fractional shares in this RWA asset");
        }

        // Dividend calculation: Allocating profits based on ownership percentage
        // Formula: (Shares / Total Pool Shares) * Incoming Profit
        let total_asset_shares = 369_000; // VORTCOIN cosmic standard fractional divisor
        let payout_usd = (user_shares * pool.total_accumulated_usd) / total_asset_shares;
        
        // Conversion of real-world USD to native VORTCOIN via the Oracle Market Price
        let vort_reward_nano = payout_usd * pool.buyback_rate_vort;

        // Add VORT coins from dividends to the user's main wallet balance
        account.balance += vort_reward_nano;
        
        // Commit the latest data state back to the Sled DB disk
        storage.save_account(user_address, &account)?;

        Ok(vort_reward_nano)
    }
}

