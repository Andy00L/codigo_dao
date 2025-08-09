use anchor_lang::prelude::*;

#[error_code]
pub enum ReputationError {
    #[msg("Invalid interaction type")]
    InvalidInteractionType,
    #[msg("Weight value too high")]
    WeightTooHigh,
    #[msg("Metadata string too long")]
    MetadataTooLong,
    #[msg("Cooldown period still active")]
    CooldownActive,
    #[msg("Cannot interact with yourself")]
    SelfInteractionForbidden,
    #[msg("Daily interaction limit exceeded")]
    DailyLimitExceeded,
    #[msg("Insufficient reputation for this action")]
    InsufficientReputation,
    #[msg("Suspicious activity detected")]
    SuspiciousActivity,
    #[msg("Bot behavior detected")]
    BotDetected,
    #[msg("Not a member of this realm")]
    NotRealmMember,
    #[msg("Admin privileges required")]
    AdminRequired,
    #[msg("Invalid action type")]
    InvalidActionType,
    #[msg("Realm name too long")]
    RealmNameTooLong,
    #[msg("Invalid algorithm weights")]
    InvalidAlgorithmWeights,
    #[msg("Badge already claimed")]
    BadgeAlreadyClaimed,
    #[msg("Invalid badge proof")]
    InvalidBadgeProof,
    #[msg("Delegation percentage too high")]
    DelegationTooHigh,
    #[msg("Cannot delegate to yourself")]
    SelfDelegationForbidden,
    #[msg("Bridge operation failed")]
    BridgeOperationFailed,
    #[msg("Source realm not found")]
    SourceRealmNotFound,
    #[msg("Cross-realm operations disabled")]
    CrossRealmDisabled,
    #[msg("Mathematical overflow")]
    MathOverflow,
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    #[msg("Profile not initialized")]
    ProfileNotInitialized,
    #[msg("Realm creation failed")]
    RealmCreationFailed,
}