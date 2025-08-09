use anchor_lang::prelude::*;
use crate::state::{reputation_profile::ReputationProfile, governance_realm::GovernanceRealm};
use crate::errors::ReputationError;

#[derive(Accounts)]
pub struct BridgeReputation<'info> {
    #[account(mut)]
    pub profile: Account<'info, ReputationProfile>,
    pub realm: Account<'info, GovernanceRealm>,
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<BridgeReputation>, _source_realm: Pubkey, bridge_weight: u8) -> Result<()> {
    require!(ctx.accounts.realm.cross_realm_enabled, ReputationError::CrossRealmDisabled);
    require!(bridge_weight > 0, ReputationError::BridgeOperationFailed);

    let factor = ctx.accounts.realm.reputation_algorithm.cross_realm_factor as u64;
    let add = (bridge_weight as u64).saturating_mul(factor);
    let profile = &mut ctx.accounts.profile;

    profile.cross_dao_reputation = profile.cross_dao_reputation.saturating_add(add);
    // small total score boost based on bridged reputation
    profile.total_score = profile.total_score.saturating_add(add / 2);

    Ok(())
}