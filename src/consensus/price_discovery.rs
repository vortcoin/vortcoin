use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketGenesisParam {
    pub current_network_difficulty: u32,
    pub estimated_hardware_cost_usd_daily: f64,
}

pub struct VortcoinPriceDiscovery;

impl VortcoinPriceDiscovery {
    /// Menghitung estimasi harga dasar intrinsik VORT berdasarkan kesulitan komputasi node
    pub fn calculate_intrinsic_floor_price(params: &MarketGenesisParam, current_block_reward: u64) -> f64 {
        // Conversion of the Nano-Vort block reward into whole units
        let reward_vort_unit = current_block_reward as f64 / 1_000_000_000.0;
        
        // Assumption of total blocks completed by the global network in one day (30 seconds per block)
        let total_blocks_daily = 2880.0;
        let total_vort_emitted_daily = total_blocks_daily * reward_vort_unit;

        // Network difficulty factor (As more global validators join, the difficulty level increases)
        let difficulty_multiplier = params.current_network_difficulty as f64;

        // Base price formula: Total daily energy expenditure divided by the total newly issued supply
        let total_energy_cost_global = params.estimated_hardware_cost_usd_daily * difficulty_multiplier;
        
        total_energy_cost_global / total_vort_emitted_daily
    }
}
