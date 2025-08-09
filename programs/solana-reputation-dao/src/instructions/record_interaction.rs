use anchor_lang::prelude::*;
use crate::state::{reputation_profile::ReputationProfile, interaction_event::InteractionEvent};
use crate::utils::{reputation_math, security};
use crate::errors::ReputationError;

#[derive(Accounts)]
pub struct RecordInteraction<'info> {
    #[account(
        mut,
        seeds = [b"reputation", from_user.key().as_ref()],
        bump = from_profile.bump
    )]
    pub from_profile: Account<'info, ReputationProfile>,

    #[account(
        mut,
        seeds = [b"reputation", to_user.key().as_ref()],
        bump = to_profile.bump
    )]
    pub to_profile: Account<'info, ReputationProfile>,

    #[account(
        init,
        payer = from_user,
        space = InteractionEvent::LEN,
    )]
    pub interaction_event: Account<'info, InteractionEvent>,

    #[account(mut)]
    pub from_user: Signer<'info>,

    /// CHECK: validated via seeds for to_profile
    pub to_user: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RecordInteraction>,
    interaction_type: u8,
    weight: u16,
    metadata: String,
) -> Result<()> {
    require!(interaction_type <= 9, ReputationError::InvalidInteractionType);
    require!(weight > 0 && weight <= 1000, ReputationError::WeightTooHigh);
    require!(metadata.len() <= 256, ReputationError::MetadataTooLong);

    let clock = Clock::get()?;
    let cooldown_period: i64 = match interaction_type {
        0..=2 => 300,
        3..=6 => 1800,
        7..=9 => 7200,
        _ => return err!(ReputationError::InvalidInteractionType),
    };

    security::validate_interaction_limits(
        &ctx.accounts.from_profile,
        &ctx.accounts.to_profile,
        interaction_type,
        clock.unix_timestamp,
        cooldown_period,
    )?;

    // Calculate reputation delta
    let mut reputation_delta = reputation_math::calculate_interaction_impact(
        &ctx.accounts.from_profile,
        &ctx.accounts.to_profile,
        interaction_type,
        weight,
    )?;

    // AI multiplier bonus
    let ai_score = ctx.accounts.from_profile.ai_validation_score;
    let ai_multiplier: u64 = if ai_score > 800 {
        110
    } else if ai_score > 500 {
        105
    } else {
        100
    };

    reputation_delta = (reputation_delta * ai_multiplier) / 100;

    // Apply trust multiplier (fixed-point where 100 == 1.00x)
    let trust_mult = ctx.accounts.from_profile.trust_multiplier.max(1);
    reputation_delta = (reputation_delta * trust_mult) / 100;

    // Update receiver profile primarily
    let to_profile = &mut ctx.accounts.to_profile;
    to_profile.total_score = to_profile.total_score.saturating_add(reputation_delta);
    to_profile.interaction_count = to_profile.interaction_count.saturating_add(1);
    to_profile.last_activity = clock.unix_timestamp;

    // Update category score based on interaction type
    let cat_index = match interaction_type {
        3 | 4 => 0, // Development
        2 | 5 | 8 => 2, // Community/Mentorship
        1 => 2, // comment -> Community
        6 => 4, // Security finding
        7 => 3, // Innovation
        9 => 1, // treat as governance/cross-chain
        _ => 2, // default Community
    };
    if (cat_index as usize) < to_profile.category_scores.len() {
        to_profile.category_scores[cat_index as usize] =
            to_profile.category_scores[cat_index as usize].saturating_add(reputation_delta);
    }

    // Update sender activity
    let from_profile = &mut ctx.accounts.from_profile;
    from_profile.last_activity = clock.unix_timestamp;

    // Fill interaction event
    let meta_hash = reputation_math::hash_metadata(&metadata);
    let event = &mut ctx.accounts.interaction_event;
    event.from = ctx.accounts.from_user.key();
    event.to = ctx.accounts.to_user.key();
    event.interaction_type = interaction_type;
    event.weight = weight;
    event.metadata_hash = meta_hash;
    event.reputation_delta = reputation_delta;
    event.timestamp = clock.unix_timestamp;
    event.bump = 0;

    emit!(InteractionRecorded {
        from: ctx.accounts.from_user.key(),
        to: ctx.accounts.to_user.key(),
        interaction_type,
        reputation_delta,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct InteractionRecorded {
    pub from: Pubkey,
    pub to: Pubkey,
    pub interaction_type: u8,
    pub reputation_delta: u64,
    pub timestamp: i64,
}