use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::{state::*, errors::*};

#[derive(Accounts)]
pub struct CloseFundraise<'info> {
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    
    #[account(mut)]
    pub dao_treasury: Account<'info, TokenAccount>,
    
    /// CHECK: PDA treasury authority
    #[account(seeds = [b"treasury"], bump)]
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseFundraise>) -> Result<()> {
    let state = &mut ctx.accounts.dao_state;
    let now = Clock::get()?.unix_timestamp;

    require!(
        now > state.fundraise_end_ts || 
        ctx.accounts.admin.key() == ctx.accounts.dao_state.key(),
        DaoError::FundraiseNotEnded
    );

    require!(!state.fundraise_ended, DaoError::FundraiseEnded);

    // Set fundraise as ended
    state.fundraise_ended = true;

    // Here we would typically:
    // 1. Calculate how much SOL to allocate to DeFi investments (90%)
    // 2. Calculate how much SOL to allocate to AMM liquidity (10%)
    // 3. Make the necessary transfers or CPI calls to set up AMM pools
    
    let total_sol = ctx.accounts.dao_treasury.amount;
    let defi_allocation = total_sol.checked_mul(90).unwrap().checked_div(100).unwrap();
    let amm_allocation = total_sol.checked_mul(10).unwrap().checked_div(100).unwrap();

    msg!("Fundraise closed. Total raised: {} SOL", total_sol);
    msg!("DeFi allocation: {} SOL", defi_allocation);
    msg!("AMM allocation: {} SOL", amm_allocation);

    Ok(())
} 