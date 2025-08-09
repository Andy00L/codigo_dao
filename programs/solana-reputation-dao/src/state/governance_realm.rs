use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GovernanceRealm {
    pub realm_id: Pubkey,                       // 32
    pub name: [u8; 32],                         // 32
    pub admin_wallets: [Pubkey; 3],             // 96
    pub reputation_algorithm: ReputationAlgorithm, // 13
    pub total_members: u32,                     // 4
    pub active_proposals: u16,                  // 2
    pub treasury_balance: u64,                  // 8
    pub governance_token: Option<Pubkey>,       // 33
    pub min_reputation_threshold: u64,          // 8
    pub voting_period_seconds: u32,             // 4
    pub cross_realm_enabled: bool,              // 1
    pub ai_moderation_enabled: bool,            // 1
    pub created_at: i64,                        // 8
    pub bump: u8,                                // 1
}
impl GovernanceRealm {
    pub const LEN: usize = 8 + 32 + 32 + 96 + 13 + 4 + 2 + 8 + 33 + 8 + 4 + 1 + 1 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ReputationAlgorithm {
    pub development_weight: u16,
    pub governance_weight: u16,
    pub community_weight: u16,
    pub innovation_weight: u16,
    pub security_weight: u16,
    pub decay_factor: u8,
    pub ai_enhancement: bool,
    pub cross_realm_factor: u8,
}