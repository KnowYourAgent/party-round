use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{state::*, errors::*};

#[derive(Accounts)]
pub struct ContributeFunds<'info> {
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    
    #[account(mut)]
    pub dao_treasury: Account<'info, TokenAccount>,
    
    /// CHECK: PDA treasury authority
    #[account(seeds = [b"treasury"], bump)]
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub contributor: Signer<'info>,
    
    #[account(
        mut,
        constraint = contributor_token_account.owner == contributor.key(),
    )]
    pub contributor_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: System program will handle SOL transfer
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ContributeFunds>, amount: u64) -> Result<()> {
    let state = &mut ctx.accounts.dao_state;
    let now = Clock::get()?.unix_timestamp;

    require!(
        now <= state.fundraise_end_ts,
        DaoError::FundraiseEnded
    );

    require!(
        !state.fundraise_ended,
        DaoError::FundraiseEnded
    );

    if !state.allowlisted_addresses.is_empty() {
        require!(
            state.allowlisted_addresses.contains(&ctx.accounts.contributor.key()),
            DaoError::NotAllowlisted
        );
    }

    require!(amount > 0, DaoError::InvalidContributionAmount);

    // Transfer SOL from contributor to treasury
    let transfer_ix = anchor_lang::system_program::Transfer {
        from: ctx.accounts.contributor.to_account_info(),
        to: ctx.accounts.dao_treasury.to_account_info(),
    };

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_ix,
        ),
        amount,
    )?;

    // Calculate tokens to receive
    let tokens_to_receive = amount
        .checked_mul(state.token_price_lamports)
        .ok_or(DaoError::InvalidContributionAmount)?;

    // Transfer tokens from treasury to contributor
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.dao_treasury.to_account_info(),
                to: ctx.accounts.contributor_token_account.to_account_info(),
                authority: ctx.accounts.treasury_authority.to_account_info(),
            },
            &[&[b"treasury", &[*ctx.bumps.get("treasury_authority").unwrap()]]]
        ),
        tokens_to_receive,
    )?;

    // Update state
    state.total_contributions = state
        .total_contributions
        .checked_add(amount)
        .ok_or(DaoError::InvalidContributionAmount)?;
    state.total_contributors += 1;

    Ok(())
} 