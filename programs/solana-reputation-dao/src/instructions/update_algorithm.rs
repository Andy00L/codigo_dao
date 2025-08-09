use anchor_lang::prelude::*;
use crate::state::GovernanceRealm;
use crate::errors::ReputationError;

#[derive(Accounts)]
pub struct UpdateAlgorithm<'info> {
    #[account(mut)]
    pub realm: Account<'info, GovernanceRealm>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateAlgorithm>,
    algorithm_weights: [u16; 5],
    decay_factor: u8,
    ai_enhancement: bool,
    cross_realm_factor: u8,
) -> Result<()> {
    require!(
        ctx.accounts.realm.admin_wallets.contains(&ctx.accounts.admin.key()),
        ReputationError::AdminRequired
    );

    let algo = &mut ctx.accounts.realm.reputation_algorithm;
    algo.development_weight = algorithm_weights[0];
    algo.governance_weight = algorithm_weights[1];
    algo.community_weight = algorithm_weights[2];
    algo.innovation_weight = algorithm_weights[3];
    algo.security_weight = algorithm_weights[4];
    algo.decay_factor = decay_factor;
    algo.ai_enhancement = ai_enhancement;
    algo.cross_realm_factor = cross_realm_factor;

    Ok(())
}