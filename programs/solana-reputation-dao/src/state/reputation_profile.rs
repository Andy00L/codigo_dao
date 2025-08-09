use anchor_lang::prelude::*;
use crate::state::badge_system::Badge;

#[account]
#[derive(Default)]
pub struct ReputationProfile {
    pub wallet: Pubkey,                    // 32
    pub total_score: u64,                  // 8
    pub category_scores: [u64; 5],         // 40 (Dev, Gov, Community, Innovation, Security)
    pub interaction_count: u32,            // 4
    pub badges: [Badge; 10],               // 10 * 73 = 730
    pub trust_multiplier: u64,             // 8 (100 = 1.00x)
    pub last_activity: i64,                // 8
    pub reputation_decay_rate: u8,         // 1
    pub delegated_power: u64,              // 8
    pub delegation_received: u64,          // 8
    pub realm_memberships: [Pubkey; 5],    // 160
    pub ai_validation_score: u32,          // 4
    pub cross_dao_reputation: u64,         // 8
    pub bump: u8,                          // 1
}
impl ReputationProfile {
    pub const LEN: usize = 8 + 32 + 8 + 40 + 4 + 730 + 8 + 8 + 1 + 8 + 8 + 160 + 4 + 8 + 1;
}