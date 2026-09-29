// =========================================================================
// VORTCOIN CORE - PROOF OF ADAPTIVE VELOCITY (PoAV) CONSENSUS ENGINE
// =========================================================================

pub struct PoAVConsensus;

impl PoAVConsensus {
    /// Calculates the difficulty level adaptively
    /// Ensures the block generation rhythm remains stable around the 30-second target
    pub fn calculate_adaptive_difficulty(
        current_difficulty: u32,
        actual_block_time_secs: u64,
        target_block_time_secs: u64
    ) -> u32 {
        let variance_threshold = 5; // 5-second tolerance (remains stable between 25s and 35s)
        let max_difficulty_cap = 32; 
        let min_difficulty_floor = 12;

        if actual_block_time_secs < (target_block_time_secs.saturating_sub(variance_threshold)) {
            // Block mined too quickly (< 25 seconds); increase the difficulty level
            if current_difficulty < max_difficulty_cap {
                println!(
                    "[PoAV INTERCEPTOR] Velocity spike ({}s < {}s). Increasing difficulty: {} -> {}", 
                    actual_block_time_secs, target_block_time_secs, current_difficulty, current_difficulty + 1
                );
                return current_difficulty + 1;
            }
        } else if actual_block_time_secs > (target_block_time_secs + variance_threshold) {
            // Block took too long to mine (> 35 seconds); lower the difficulty level
            if current_difficulty > min_difficulty_floor {
                println!(
                    "[PoAV INTERCEPTOR] Network latency ({}s > {}s). Reducing difficulty: {} -> {}", 
                    actual_block_time_secs, target_block_time_secs, current_difficulty, current_difficulty - 1
                );
                return current_difficulty - 1;
            }
        }

        // Remain at the same difficulty level if within the 25–35 second target range
        current_difficulty
    }

    /// Calculating the estimated average hash rate required to complete PoAV
    pub fn estimated_hashes_for_difficulty(difficulty: u32) -> u64 {
        1u64 << difficulty
    }
}