use anchor_lang::prelude::*;

#[account]
pub struct InteractionEvent {
    pub from: Pubkey,               // 32
    pub to: Pubkey,                 // 32
    pub interaction_type: u8,       // 1
    pub weight: u16,                // 2
    pub metadata_hash: [u8; 32],    // 32
    pub reputation_delta: u64,      // 8
    pub timestamp: i64,             // 8
    pub bump: u8,                   // 1
}
impl InteractionEvent {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 2 + 32 + 8 + 8 + 1;
}
