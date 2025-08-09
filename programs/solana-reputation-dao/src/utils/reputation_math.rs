use anchor_lang::prelude::*;
use crate::state::ReputationProfile;
use crate::errors::ReputationError;

pub fn calculate_interaction_impact(
    from_profile: &ReputationProfile,
    to_profile: &ReputationProfile,
    interaction_type: u8,
    weight: u16,
) -> Result<u64> {
    let base_delta: u64 = match interaction_type {
        0 => 10,
        1 => 5,
        2 => 15,
        3 => 25,
        4 => 50,
        5 => 30,
        6 => 40,
        7 => 75,
        8 => 60,
        9 => 20,
        _ => return err!(ReputationError::InvalidInteractionType),
    };

    let weighted_delta = (base_delta.saturating_mul(weight as u64)) / 100;

    let giver_multiplier = influence_multiplier(from_profile.total_score);
    let receiver_resistance = resistance_factor(to_profile.total_score);
    let time_bonus = activity_bonus(from_profile.last_activity)?;

    let mut final_delta = weighted_delta
        .saturating_mul(giver_multiplier)
        .saturating_mul(time_bonus)
        .saturating_div(receiver_resistance);

    if final_delta > 200 {
        final_delta = 200;
    }

    Ok(final_delta)
}

fn influence_multiplier(reputation: u64) -> u64 {
    match reputation {
        0..=100 => 80,
        101..=500 => 100,
        501..=1000 => 120,
        1001..=5000 => 140,
        _ => 160,
    }
}

fn resistance_factor(reputation: u64) -> u64 {
    match reputation {
        0..=100 => 80,
        101..=500 => 100,
        501..=1000 => 120,
        1001..=5000 => 140,
        _ => 180,
    }
}

fn activity_bonus(last_activity: i64) -> Result<u64> {
    let current_time = Clock::get()?.unix_timestamp;
    let elapsed = current_time.saturating_sub(last_activity);
    let bonus = match elapsed {
        0..=3600 => 110,
        3601..=86400 => 105,
        86401..=604800 => 100,
        604801..=2592000 => 95,
        _ => 85,
    };
    Ok(bonus)
}

pub fn calculate_reputation_decay(
    profile: &mut ReputationProfile,
    current_timestamp: i64,
) -> Result<()> {
    let elapsed = current_timestamp.saturating_sub(profile.last_activity);
    let days = (elapsed / 86_400).max(0);
    if days > 7 {
        let rate = profile.reputation_decay_rate as u64;
        let decay_total = profile.total_score
            .saturating_mul(rate)
            .saturating_mul(days as u64)
            / (100 * 30);
        profile.total_score = profile.total_score.saturating_sub(decay_total);

        for i in 0..5 {
            let category_decay = profile.category_scores[i]
                .saturating_mul(rate)
                .saturating_mul(days as u64)
                / (100 * 30);
            profile.category_scores[i] = profile.category_scores[i].saturating_sub(category_decay);
        }
    }
    Ok(())
}

pub fn hash_metadata(metadata: &str) -> [u8; 32] {
    use anchor_lang::solana_program::keccak;
    let hash = keccak::hash(metadata.as_bytes());
    hash.to_bytes()
}