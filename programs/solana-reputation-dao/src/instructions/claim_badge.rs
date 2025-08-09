use anchor_lang::prelude::*;
use crate::state::{ReputationProfile, Badge, BadgeType, BadgeReceipt};
use crate::errors::ReputationError;
use crate::utils::constants::*;

#[derive(Accounts)]
pub struct ClaimBadge<'info> {
    #[account(mut, seeds = [b"reputation", user.key().as_ref()], bump = profile.bump)]
    pub profile: Account<'info, ReputationProfile>,
    #[account(init, payer = user, space = BadgeReceipt::LEN)]
    pub badge_account: Account<'info, BadgeReceipt>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimBadge>, badge_type: u8, proof_hash: [u8; 32]) -> Result<()> {
    let clock = Clock::get()?;
    let bt: BadgeType = BadgeType::try_from(badge_type).map_err(|_| error!(ReputationError::InvalidBadgeProof))?;

    if ctx.accounts.profile.badges.iter().any(|b| b.badge_type == bt) {
        return err!(ReputationError::BadgeAlreadyClaimed);
    }

    let receipt = &mut ctx.accounts.badge_account;
    receipt.owner = ctx.accounts.user.key();
    receipt.badge_type = badge_type;
    receipt.proof_hash = proof_hash;
    receipt.earned_at = clock.unix_timestamp;
    receipt.bump = 0;

    let profile = &mut ctx.accounts.profile;
    if let Some(slot) = profile.badges.iter_mut().find(|b| matches!(b.badge_type, BadgeType::None)) {
        *slot = Badge {
            badge_type: bt,
            earned_at: clock.unix_timestamp,
            issuer_realm: Pubkey::default(),
            metadata_hash: proof_hash,
        };
    } else {
        let last = profile.badges.last_mut().unwrap();
        *last = Badge {
            badge_type: bt,
            earned_at: clock.unix_timestamp,
            issuer_realm: Pubkey::default(),
            metadata_hash: proof_hash,
        };
    }

    profile.total_score = profile.total_score.saturating_add(BADGE_SCORE_BONUS);
    let idx = match bt {
        BadgeType::Developer => 0,
        BadgeType::GovernanceParticipant => 1,
        BadgeType::CommunityBuilder | BadgeType::Mentor | BadgeType::EarlyAdopter => 2,
        BadgeType::Innovation | BadgeType::AIValidator => 3,
        BadgeType::SecurityAuditor => 4,
        BadgeType::CrossChainBridge | BadgeType::Custom | BadgeType::None => 2,
    };
    profile.category_scores[idx] = profile.category_scores[idx].saturating_add(BADGE_SCORE_BONUS);

    Ok(())
}