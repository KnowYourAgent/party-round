use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeDaoParams {
    pub token_name: String,
    pub token_symbol: String,
    pub total_supply: u64,
    pub fundraise_end_ts: i64,
    pub token_price_lamports: u64,
    pub allowlisted_addresses: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(params: InitializeDaoParams)]
pub struct InitializeDao<'info> {
    #[account(init, payer = payer, space = 8 + DaoState::MAX_SIZE)]
    pub dao_state: Account<'info, DaoState>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority.key()
    )]
    pub dao_mint: Account<'info, Mint>,
    
    /// CHECK: PDA used as mint authority
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = payer,
        token::mint = dao_mint,
        token::authority = treasury_authority.key()
    )]
    pub dao_treasury: Account<'info, TokenAccount>,
    
    /// CHECK: PDA used as treasury authority
    #[account(seeds = [b"treasury"], bump)]
    pub treasury_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeDao>, params: InitializeDaoParams) -> Result<()> {
    let state = &mut ctx.accounts.dao_state;
    
    state.token_name = params.token_name;
    state.token_symbol = params.token_symbol;
    state.total_supply = params.total_supply;
    state.fundraise_end_ts = params.fundraise_end_ts;
    state.token_price_lamports = params.token_price_lamports;
    state.allowlisted_addresses = params.allowlisted_addresses;
    state.fundraise_ended = false;

    // Mint initial supply to treasury
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.dao_mint.to_account_info(),
                to: ctx.accounts.dao_treasury.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[b"mint_authority", &[*ctx.bumps.get("mint_authority").unwrap()]]]
        ),
        params.total_supply,
    )?;

    Ok(())
} 