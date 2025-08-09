use anchor_lang::prelude::*;
use crate::state::{reputation_profile::ReputationProfile, governance_realm::GovernanceRealm};
use crate::errors::ReputationError;

pub fn validate_interaction_limits(
    from_profile: &ReputationProfile,
    to_profile: &ReputationProfile,
    interaction_type: u8,
    current_timestamp: i64,
    cooldown_period: i64,
) -> Result<()> {
    // Cooldown except for high reputation users
    let since = current_timestamp.saturating_sub(from_profile.last_activity);
    require!(
        since >= cooldown_period || from_profile.total_score > 1000,
        ReputationError::CooldownActive
    );

    // Prevent self-interaction
    require!(from_profile.wallet != to_profile.wallet, ReputationError::SelfInteractionForbidden);

    // Daily limits
    let daily_limit = calculate_daily_limit(from_profile.total_score);
    let interactions_today = count_daily_interactions(from_profile, current_timestamp);
    require!(interactions_today < daily_limit, ReputationError::DailyLimitExceeded);

    // Optional: additional permission checks by interaction type
    validate_interaction_permissions(from_profile, interaction_type)?;

    Ok(())
}

fn calculate_daily_limit(reputation: u64) -> u32 {
    match reputation {
        0..=100 => 5,
        101..=500 => 15,
        501..=1000 => 30,
        1001..=5000 => 50,
        _ => 100,
    }
}

fn count_daily_interactions(profile: &ReputationProfile, current_timestamp: i64) -> u32 {
    let seconds_in_day = 86_400;
    let elapsed = current_timestamp.saturating_sub(profile.last_activity);
    if elapsed > seconds_in_day { 0 } else { profile.interaction_count % 100 }
}

fn validate_interaction_permissions(profile: &ReputationProfile, interaction_type: u8) -> Result<()> {
    match interaction_type {
        0..=2 => Ok(()),
        3..=5 => {
            require!(profile.total_score >= 100, ReputationError::InsufficientReputation);
            Ok(())
        },
        6..=8 => {
            require!(profile.total_score >= 500, ReputationError::InsufficientReputation);
            Ok(())
        },
        9 => {
            require!(profile.total_score >= 1000 && profile.cross_dao_reputation > 0, ReputationError::InsufficientReputation);
            Ok(())
        },
        _ => Err(error!(ReputationError::InvalidInteractionType)),
    }
}

// Optional realm action authorization
pub fn authorize_realm_action(
    profile: &ReputationProfile,
    realm: &GovernanceRealm,
    action_type: u8, // 0 vote, 1 propose, 2 admin
) -> Result<()> {
    require!(profile.total_score >= realm.min_reputation_threshold, ReputationError::InsufficientReputation);

    // membership check (simplified)
    let is_member = profile.realm_memberships.contains(&realm.realm_id);
    require!(is_member, ReputationError::NotRealmMember);

    match action_type {
        0 => Ok(()),
        1 => {
            require!(profile.total_score >= realm.min_reputation_threshold.saturating_mul(2), ReputationError::InsufficientReputation);
            Ok(())
        },
        2 => {
            require!(realm.admin_wallets.contains(&profile.wallet), ReputationError::AdminRequired);
            Ok(())
        },
        _ => Err(error!(ReputationError::InvalidActionType)),
    }
}

// Dynamic cooldown with frequency penalty and reputation relief
pub fn calculate_dynamic_cooldown(base_cooldown: i64, recent_interactions: u32, reputation: u64) -> i64 {
    let reputation_factor = if reputation > 1000 {
        50
    } else if reputation > 500 {
        75
    } else {
        100
    };

    let frequency_penalty = match recent_interactions {
        0..=5 => 100,
        6..=15 => 150,
        16..=30 => 200,
        _ => 300,
    };

    (base_cooldown * frequency_penalty * reputation_factor) / 10_000
}