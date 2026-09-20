// =========================================================================
// VORTCOIN CORE - ELASTIC DYNAMIC DIFFICULTY ADJUSTMENT SYSTEM (PoAV LOGIC)
// =========================================================================

pub struct PoAVConsensus;

impl PoAVConsensus {
    /// Dynamically computes the target block difficulty.
    /// Uses dampening factor algorithms to prevent massive spikes for retail end-users.
    pub fn calculate_adaptive_difficulty(
        current_difficulty: u32,
        actual_block_time_secs: u64,
        target_block_time_secs: u64
    ) -> u32 {
        let variance_threshold = 5; // 5 seconds grace period allowance

        let max_difficulty_cap = 32; 
        let min_difficulty_floor = 12;

        if actual_block_time_secs < (target_block_time_secs - variance_threshold) {
            // Blocks are being minted too fast, safely step up difficulty by exactly +1
            if current_difficulty < max_difficulty_cap {
                println!("[PoAV INTERCEPTOR] Network mining velocity accelerated. Incrementing complexity (+1).");
                return current_difficulty + 1;
            }
        } else if actual_block_time_secs > (target_block_time_secs + variance_threshold) {
            
            if current_difficulty > min_difficulty_floor {
                println!("[PoAV INTERCEPTOR] Network latency detected. Reducing hash complexity (-1).");
                return current_difficulty - 1;
            }
        }

        // Maintain status quo if system is within the balanced target window
        current_difficulty
    }
}
