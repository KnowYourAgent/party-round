// programs/party_round/src/instructions/lock_liquidity.rs

use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::CustomError;
use anchor_spl::token::{TokenAccount, Token, Transfer};

// Example time-based lock: no partial withdrawals until after `lock_end_ts`.
#[derive(Accounts)]
pub struct LockLiquidity<'info> {
    #[account(mut, has_one = authority)]
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub authority: Signer<'info>,  // E.g. a PDA or multi-sig authority
    #[account(mut)]
    pub liquidity_token_account: Account<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn lock_liquidity(ctx: Context<LockLiquidity>, lock_end_ts: i64) -> Result<()> {
    // Check that the current time is before the lock period ends
    let clock = Clock::get()?;
    let state = &mut ctx.accounts.dao_state;

    // Ensure we haven't already locked liquidity
    require!(
        !state.liquidity_locked,
        CustomError::LiquidityAlreadyLocked
    );

    // Store lock details in the DAO state
    state.liquidity_locked = true;
    state.lock_end_ts = lock_end_ts;

    // If we need to forcibly move tokens into a time-locked escrow account,
    // we could do that here. For example:
    //   1. Create a PDA escrow account
    //   2. Transfer LP tokens from treasury to the escrow
    //   3. No one can withdraw from the escrow until lock_end_ts

    // For demonstration, just set flags in the state
    msg!("Liquidity is now locked until timestamp: {}", lock_end_ts);

    Ok(())
}

// Example instruction to withdraw locked liquidity after the lock period
#[derive(Accounts)]
pub struct WithdrawLockedLiquidity<'info> {
    #[account(mut, has_one = authority)]
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub liquidity_token_account: Account<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn withdraw_locked_liquidity(ctx: Context<WithdrawLockedLiquidity>) -> Result<()> {
    let clock = Clock::get()?;
    let state = &mut ctx.accounts.dao_state;

    // Check lock period
    require!(
        state.liquidity_locked,
        CustomError::LiquidityNotLocked
    );
    require!(
        clock.unix_timestamp >= state.lock_end_ts,
        CustomError::LiquidityStillLocked
    );

    // Perform transfer from escrow to final destination (if escrow is used).
    // Or, if the tokens never left the treasury but were locked logically,
    // then allow an update to the state to "unlock" them.
    // (Implementation depends on our chosen approach.)

    // Example: simply unlock
    state.liquidity_locked = false;
    msg!("Liquidity withdrawn after lock ended.");

    Ok(())
}