use anchor_lang::prelude::*;
use crate::state::reputation_profile::ReputationProfile;
use crate::errors::ReputationError;

pub fn calculate_interaction_impact(
    from_profile: &ReputationProfile,
    to_profile: &ReputationProfile,
    interaction_type: u8,
    weight: u16,
) -> Result<u64> {
    let base_delta: u64 = match interaction_type {
        0 => 10,   // upvote
        1 => 5,    // comment
        2 => 15,   // helpful answer
        3 => 25,   // code review
        4 => 50,   // collaboration
        5 => 30,   // community contribution
        6 => 40,   // security finding
        7 => 75,   // major innovation
        8 => 60,   // mentorship
        9 => 20,   // cross-chain usage
        _ => return err!(ReputationError::InvalidInteractionType),
    };

    // Weight multiplier (percent)
    let weighted_delta = (base_delta.saturating_mul(weight as u64)) / 100;

    let giver_multiplier = influence_multiplier(from_profile.total_score);
    let receiver_resistance = resistance_factor(to_profile.total_score);
    let time_bonus = activity_bonus(from_profile.last_activity)?;

    let mut final_delta = weighted_delta
        .saturating_mul(giver_multiplier)
        .saturating_mul(time_bonus)
        .saturating_div(receiver_resistance);

    // cap impact
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
        0..=3600 => 110,         // 10% bonus within 1 hour
        3601..=86400 => 105,     // 5% within 1 day
        86401..=604800 => 100,   // neutral within 1 week
        604801..=2592000 => 95,  // -5% beyond 1 week
        _ => 85,                 // -15% long inactivity
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
    use solana_program::keccak;
    let hash = keccak::hash(metadata.as_bytes());
    hash.to_bytes()
}

// Simplified AI validation (mock on-chain scoring)
pub fn validate_with_ai_score(interaction_pattern: &[u8], historical_data: &[u64]) -> u32 {
    let pattern_score = pattern_legitimacy(interaction_pattern);
    let historical_score = historical_consistency(historical_data);
    let anomaly_score = anomaly_detection(interaction_pattern, historical_data);
    let combined = (pattern_score * 40 + historical_score * 35 + anomaly_score * 25) / 100;
    combined.min(1000)
}

fn pattern_legitimacy(pattern: &[u8]) -> u32 {
    let diversity = pattern.iter().fold(0u32, |acc, &x| acc ^ x as u32);
    (diversity % 400) + 300
}

fn historical_consistency(h: &[u64]) -> u32 {
    if h.is_empty() { return 500; }
    let var = variance(h).min(1_000_000);
    let score = 700u32.saturating_sub((var / 100) as u32);
    score.clamp(300, 700)
}

fn anomaly_detection(pattern: &[u8], historical: &[u64]) -> u32 {
    let pattern_sum: u32 = pattern.iter().map(|&x| x as u32).sum();
    let hist_avg = if historical.is_empty() { 0 } else { historical.iter().sum::<u64>() / historical.len() as u64 };
    let anomaly = (pattern_sum as u64).abs_diff(hist_avg * 10);
    if anomaly < 100 { 700 } else if anomaly < 1_000 { 600 } else { 450 }
}

fn variance(data: &[u64]) -> u64 {
    if data.is_empty() { return 0; }
    let mean = data.iter().sum::<u64>() / data.len() as u64;
    data.iter().map(|&x| {
        let d = if x >= mean { x - mean } else { mean - x };
        (d as u128 * d as u128) as u64
    }).sum::<u64>() / data.len() as u64
}