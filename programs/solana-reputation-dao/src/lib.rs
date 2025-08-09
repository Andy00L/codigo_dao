use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod utils;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("So1anaRepU6tWxzAoNn31M3b9nS6g2oT5VtN7k4XQ4vH");

#[program]
pub mod solana_reputation_dao {
    use super::*;

    pub fn initialize_profile(ctx: Context<initialize_profile::InitializeProfile>) -> Result<()> {
        initialize_profile::handler(ctx)
    }

    pub fn record_interaction(
        ctx: Context<record_interaction::RecordInteraction>,
        interaction_type: u8,
        weight: u16,
        metadata: String,
    ) -> Result<()> {
        record_interaction::handler(ctx, interaction_type, weight, metadata)
    }

    pub fn cast_reputation_vote(
        ctx: Context<cast_reputation_vote::CastReputationVote>,
        vote_type: u8,
        justification: String,
    ) -> Result<()> {
        cast_reputation_vote::handler(ctx, vote_type, justification)
    }

    pub fn create_realm(
        ctx: Context<create_realm::CreateRealm>,
        realm_name: String,
        algorithm_weights: [u16; 5],
    ) -> Result<()> {
        create_realm::handler(ctx, realm_name, algorithm_weights)
    }

    pub fn delegate_reputation(
        ctx: Context<delegate_reputation::DelegateReputation>,
        delegate_to: Pubkey,
        weight_percentage: u8,
    ) -> Result<()> {
        delegate_reputation::handler(ctx, delegate_to, weight_percentage)
    }

    pub fn claim_badge(
        ctx: Context<claim_badge::ClaimBadge>,
        badge_type: u8,
        proof_hash: [u8; 32],
    ) -> Result<()> {
        claim_badge::handler(ctx, badge_type, proof_hash)
    }

    pub fn bridge_reputation(
        ctx: Context<bridge_reputation::BridgeReputation>,
        source_realm: Pubkey,
        bridge_weight: u8,
    ) -> Result<()> {
        bridge_reputation::handler(ctx, source_realm, bridge_weight)
    }

    pub fn update_algorithm(
        ctx: Context<update_algorithm::UpdateAlgorithm>,
        algorithm_weights: [u16; 5],
        decay_factor: u8,
        ai_enhancement: bool,
        cross_realm_factor: u8,
    ) -> Result<()> {
        update_algorithm::handler(ctx, algorithm_weights, decay_factor, ai_enhancement, cross_realm_factor)
    }
}