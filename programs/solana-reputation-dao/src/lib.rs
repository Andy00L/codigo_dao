use anchor_lang::prelude::*;

pub mod errors;
pub mod state;
pub mod utils;
pub mod instructions;

use crate::instructions::{
    bridge_reputation::BridgeReputation,
    cast_reputation_vote::CastReputationVote,
    claim_badge::ClaimBadge,
    create_realm::CreateRealm,
    delegate_reputation::DelegateReputation,
    initialize_profile::InitializeProfile,
    record_interaction::RecordInteraction,
    update_algorithm::UpdateAlgorithm,
};

declare_id!("CiQgzVgLRjLC13vFjBnfsf6QGWnHgqGMZhrCnxZt95ha");

#[program]
pub mod solana_reputation_dao {
    use super::*;

    pub fn initialize_profile(ctx: Context<InitializeProfile>) -> Result<()> {
        instructions::initialize_profile::handler(ctx)
    }

    pub fn record_interaction(
        ctx: Context<RecordInteraction>,
        interaction_type: u8,
        weight: u16,
        metadata: String,
    ) -> Result<()> {
        instructions::record_interaction::handler(ctx, interaction_type, weight, metadata)
    }

    pub fn cast_reputation_vote(
        ctx: Context<CastReputationVote>,
        vote_type: u8,
        justification: String,
    ) -> Result<()> {
        instructions::cast_reputation_vote::handler(ctx, vote_type, justification)
    }

    pub fn create_realm(
        ctx: Context<CreateRealm>,
        realm_name: String,
        algorithm_weights: [u16; 5],
    ) -> Result<()> {
        instructions::create_realm::handler(ctx, realm_name, algorithm_weights)
    }

    pub fn delegate_reputation(
        ctx: Context<DelegateReputation>,
        delegate_to: Pubkey,
        weight_percentage: u8,
    ) -> Result<()> {
        instructions::delegate_reputation::handler(ctx, delegate_to, weight_percentage)
    }

    pub fn claim_badge(
        ctx: Context<ClaimBadge>,
        badge_type: u8,
        proof_hash: [u8; 32],
    ) -> Result<()> {
        instructions::claim_badge::handler(ctx, badge_type, proof_hash)
    }

    pub fn bridge_reputation(
        ctx: Context<BridgeReputation>,
        source_realm: Pubkey,
        bridge_weight: u8,
    ) -> Result<()> {
        instructions::bridge_reputation::handler(ctx, source_realm, bridge_weight)
    }

    pub fn update_algorithm(
        ctx: Context<UpdateAlgorithm>,
        algorithm_weights: [u16; 5],
        decay_factor: u8,
        ai_enhancement: bool,
        cross_realm_factor: u8,
    ) -> Result<()> {
        instructions::update_algorithm::handler(
            ctx,
            algorithm_weights,
            decay_factor,
            ai_enhancement,
            cross_realm_factor,
        )
    }
}