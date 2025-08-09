use anchor_lang::prelude::*;
use crate::state::{ReputationProfile, GovernanceRealm};
use crate::errors::ReputationError;

#[derive(Accounts)]
pub struct CastReputationVote<'info> {
    #[account(
        mut,
        seeds = [b"reputation", voter.key().as_ref()],
        bump = voter_profile.bump
    )]
    pub voter_profile: Account<'info, ReputationProfile>,
    #[account(mut)]
    pub realm: Account<'info, GovernanceRealm>,
    #[account(mut)]
    pub voter: Signer<'info>,
}

pub fn handler(ctx: Context<CastReputationVote>, vote_type: u8, justification: String) -> Result<()> {
    require!(justification.len() <= 280, ReputationError::MetadataTooLong);
    require!(
        ctx.accounts.voter_profile.total_score >= ctx.accounts.realm.min_reputation_threshold.min(100),
        ReputationError::InsufficientReputation
    );
    let increment: u64 = match vote_type {
        0 => 5,
        1 | 2 => 15,
        _ => return err!(ReputationError::InvalidActionType),
    };
    let voter_profile = &mut ctx.accounts.voter_profile;
    voter_profile.total_score = voter_profile.total_score.saturating_add(increment);
    voter_profile.category_scores[1] =
        voter_profile.category_scores[1].saturating_add(increment);

    Ok(())
}