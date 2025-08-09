use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BadgeType {
    None,
    Developer,
    GovernanceParticipant,
    CommunityBuilder,
    SecurityAuditor,
    Innovation,
    Mentor,
    EarlyAdopter,
    CrossChainBridge,
    AIValidator,
    Custom,
}
impl Default for BadgeType {
    fn default() -> Self { BadgeType::None }
}
impl TryFrom<u8> for BadgeType {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0 => BadgeType::None,
            1 => BadgeType::Developer,
            2 => BadgeType::GovernanceParticipant,
            3 => BadgeType::CommunityBuilder,
            4 => BadgeType::SecurityAuditor,
            5 => BadgeType::Innovation,
            6 => BadgeType::Mentor,
            7 => BadgeType::EarlyAdopter,
            8 => BadgeType::CrossChainBridge,
            9 => BadgeType::AIValidator,
            10 => BadgeType::Custom,
            _ => return Err(())
        })
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct Badge {
    pub badge_type: BadgeType,      // 1 byte (borsh enum)
    pub earned_at: i64,             // 8 bytes
    pub issuer_realm: Pubkey,       // 32 bytes
    pub metadata_hash: [u8; 32],    // 32 bytes
}

#[account]
pub struct BadgeReceipt {
    pub owner: Pubkey,              // 32
    pub badge_type: u8,             // 1
    pub proof_hash: [u8; 32],       // 32
    pub earned_at: i64,             // 8
    pub bump: u8,                   // 1
}
impl BadgeReceipt {
    pub const LEN: usize = 8 + 32 + 1 + 32 + 8 + 1;
}