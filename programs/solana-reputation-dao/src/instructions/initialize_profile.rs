use anchor_lang::prelude::*;
use crate::state::reputation_profile::ReputationProfile;
use crate::utils::constants::*;

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = user,
        space = ReputationProfile::LEN,
        seeds = [b"reputation", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, ReputationProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeProfile>) -> Result<()> {
    let clock = Clock::get()?;
    let profile = &mut ctx.accounts.profile;

    profile.wallet = ctx.accounts.user.key();
    profile.total_score = 0;
    profile.category_scores = [0u64; 5];
    profile.interaction_count = 0;
    profile.badges = [Default::default(); MAX_BADGES as usize];
    profile.trust_multiplier = DEFAULT_TRUST_MULTIPLIER as u64; // 100 = 1.00x
    profile.last_activity = 0; // allow immediate first interaction
    profile.reputation_decay_rate = DEFAULT_DECAY_RATE;
    profile.delegated_power = 0;
    profile.delegation_received = 0;
    profile.realm_memberships = [Pubkey::default(); 5];
    profile.ai_validation_score = 500;
    profile.cross_dao_reputation = 0;
    profile.bump = *ctx.bumps.get("profile").unwrap();

    // touch for created_at maybe
    let _ = clock.unix_timestamp;

    Ok(())
}