use anchor_lang::prelude::*;
use crate::state::ReputationProfile;
use crate::errors::ReputationError;

#[derive(Accounts)]
pub struct DelegateReputation<'info> {
    #[account(mut, seeds = [b"reputation", delegator.key().as_ref()], bump = delegator_profile.bump)]
    pub delegator_profile: Account<'info, ReputationProfile>,
    #[account(mut)]
    pub delegatee_profile: Account<'info, ReputationProfile>,
    #[account(mut)]
    pub delegator: Signer<'info>,
}

pub fn handler(
    ctx: Context<DelegateReputation>,
    delegate_to: Pubkey,
    weight_percentage: u8,
) -> Result<()> {
    require!(weight_percentage > 0 && weight_percentage <= 100, ReputationError::DelegationTooHigh);

    let delegator_profile = &mut ctx.accounts.delegator_profile;
    require!(delegator_profile.wallet != delegate_to, ReputationError::SelfDelegationForbidden);
    require!(ctx.accounts.delegatee_profile.wallet == delegate_to, ReputationError::ProfileNotInitialized);

    let power = (delegator_profile.total_score as u128 * weight_percentage as u128 / 100) as u64;

    let prev = delegator_profile.delegated_power;
    delegator_profile.delegated_power = power;

    let delegatee_profile = &mut ctx.accounts.delegatee_profile;
    delegatee_profile.delegation_received =
        delegatee_profile.delegation_received.saturating_add(power.saturating_sub(prev));

    Ok(())
}