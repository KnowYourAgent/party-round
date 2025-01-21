// programs/party_round/src/instructions/multisig_governance.rs

use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + MultisigConfig::MAX_SIZE
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    pub system_program: Program<'info, System>,
}

pub fn create_multisig(ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u8) -> Result<()> {
    let config = &mut ctx.accounts.multisig_config;
    require!(threshold as usize <= owners.len(), CustomError::InvalidThreshold);

    config.owners = owners;
    config.threshold = threshold;
    config.owner_set_seqno = 1;

    msg!("Multisig created. Threshold: {}", threshold);
    Ok(())
}

#[derive(Accounts)]
pub struct ProposeAction<'info> {
    #[account(mut, has_one = proposer)]
    pub multisig_config: Account<'info, MultisigConfig>,
    #[account(mut)]
    pub proposer: Signer<'info>,
}

pub fn propose_action(ctx: Context<ProposeAction>, description: String) -> Result<()> {
    let config = &mut ctx.accounts.multisig_config;
    // In a real scenario, we'd store a record of the proposed action 
    // (which function, parameters, etc.) in an array or dedicated data structure.
    msg!("New action proposed: {}", description);
    Ok(())
}

#[derive(Accounts)]
pub struct ApproveAction<'info> {
    #[account(mut)]
    pub multisig_config: Account<'info, MultisigConfig>,
    #[account()]
    pub owner: Signer<'info>, // Must be one of the owners
}

pub fn approve_action(ctx: Context<ApproveAction>) -> Result<()> {
    let config = &mut ctx.accounts.multisig_config;

    // Check owner is valid
    require!(
        config.owners.contains(&ctx.accounts.owner.key()),
        CustomError::Unauthorized
    );

    // For real usage: increment approval count for the given proposal.
    // If approvals >= threshold, mark the proposal as "executable".

    // Example: We'll just log the approval. In real code, track the state in an array or map.
    msg!("Action approved by owner: {}", ctx.accounts.owner.key());
    Ok(())
}