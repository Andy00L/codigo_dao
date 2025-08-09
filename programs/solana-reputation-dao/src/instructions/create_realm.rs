use anchor_lang::prelude::*;
use crate::state::governance_realm::{GovernanceRealm, ReputationAlgorithm};
use crate::errors::ReputationError;

#[derive(Accounts)]
#[instruction(realm_name: String, algorithm_weights: [u16; 5])]
pub struct CreateRealm<'info> {
    #[account(
        init,
        payer = admin,
        space = GovernanceRealm::LEN,
        seeds = [b"realm", realm_name.as_bytes()],
        bump
    )]
    pub realm: Account<'info, GovernanceRealm>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateRealm>,
    realm_name: String,
    algorithm_weights: [u16; 5],
) -> Result<()> {
    require!(realm_name.len() <= 32, ReputationError::RealmNameTooLong);

    // Basic validation
    let total_weight: u64 = algorithm_weights.iter().map(|&x| x as u64).sum();
    require!(total_weight > 0, ReputationError::InvalidAlgorithmWeights);

    let clock = Clock::get()?;
    let realm = &mut ctx.accounts.realm;

    // Fill fields
    realm.realm_id = ctx.accounts.realm.key();
    let mut name_buf = [0u8; 32];
    let name_bytes = realm_name.as_bytes();
    name_buf[..name_bytes.len()].copy_from_slice(name_bytes);
    realm.name = name_buf;
    realm.admin_wallets = [ctx.accounts.admin.key(), Pubkey::default(), Pubkey::default()];
    realm.reputation_algorithm = ReputationAlgorithm {
        development_weight: algorithm_weights[0],
        governance_weight: algorithm_weights[1],
        community_weight: algorithm_weights[2],
        innovation_weight: algorithm_weights[3],
        security_weight: algorithm_weights[4],
        decay_factor: 2, // default
        ai_enhancement: true,
        cross_realm_factor: 10,
    };
    realm.total_members = 0;
    realm.active_proposals = 0;
    realm.treasury_balance = 0;
    realm.governance_token = None;
    realm.min_reputation_threshold = 50;
    realm.voting_period_seconds = 3 * 24 * 3600;
    realm.cross_realm_enabled = true;
    realm.ai_moderation_enabled = false;
    realm.created_at = clock.unix_timestamp;
    realm.bump = *ctx.bumps.get("realm").unwrap();

    Ok(())
}