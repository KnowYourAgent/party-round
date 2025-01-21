use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Burn};
use crate::{state::*, errors::*};

#[derive(Accounts)]
pub struct RedeemTokens<'info> {
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    
    #[account(mut)]
    pub dao_treasury: Account<'info, TokenAccount>,
    
    /// CHECK: PDA treasury authority
    #[account(seeds = [b"treasury"], bump)]
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub redeemer: Signer<'info>,
    
    #[account(
        mut,
        constraint = redeemer_token_account.owner == redeemer.key(),
    )]
    pub redeemer_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub dao_mint: Account<'info, token::Mint>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RedeemTokens>, amount: u64) -> Result<()> {
    let state = &ctx.accounts.dao_state;
    
    require!(
        state.fundraise_ended,
        DaoError::FundraiseNotEnded
    );

    require!(
        amount > 0 && 
        amount <= ctx.accounts.redeemer_token_account.amount,
        DaoError::InvalidRedemptionAmount
    );

    // Calculate proportional share of treasury
    let total_supply = ctx.accounts.dao_mint.supply;
    let treasury_balance = ctx.accounts.dao_treasury.amount;
    
    let redemption_amount = (treasury_balance as u128)
        .checked_mul(amount as u128)
        .unwrap()
        .checked_div(total_supply as u128)
        .unwrap() as u64;

    require!(
        redemption_amount > 0 && 
        redemption_amount <= treasury_balance,
        DaoError::InsufficientTreasuryFunds
    );

    // Transfer SOL from treasury to redeemer
    let transfer_ix = anchor_lang::system_program::Transfer {
        from: ctx.accounts.dao_treasury.to_account_info(),
        to: ctx.accounts.redeemer.to_account_info(),
    };

    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            transfer_ix,
        ),
        redemption_amount,
    )?;

    // Burn the redeemed tokens
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.dao_mint.to_account_info(),
                from: ctx.accounts.redeemer_token_account.to_account_info(),
                authority: ctx.accounts.redeemer.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!("Redeemed {} tokens for {} SOL", amount, redemption_amount);
    
    Ok(())
} 