pub mod instructions;
pub mod state;
pub mod errors;

use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::DaoError;

declare_id!("Fg6PaFpoGXkYsidMpWxqSWY79c2JEvWC7jg4fQtMaH8q");

#[account]
#[derive(Default)]
pub struct ReentrancyGuard {
    pub in_progress: bool,
}

impl ReentrancyGuard {
    pub fn start(&mut self) -> Result<()> {
        require!(!self.in_progress, DaoError::ReentrancyAttempt);
        self.in_progress = true;
        Ok(())
    }

    pub fn complete(&mut self) {
        self.in_progress = false;
    }
}

// Add this trait to share reentrancy guard account validation
pub trait GuardedInstruction<'info> {
    fn reentrancy_guard(&self) -> &Account<'info, ReentrancyGuard>;
}

#[program]
pub mod party_round {
    use super::*;

    pub fn initialize_dao(
        ctx: Context<InitializeDao>,
        params: InitializeDaoParams,
    ) -> Result<()> {
        ctx.accounts.reentrancy_guard().start()?;
        let result = instructions::initialize_dao::handler(ctx, params);
        ctx.accounts.reentrancy_guard().complete();
        result
    }

    pub fn contribute_funds(
        ctx: Context<ContributeFunds>, 
        amount: u64
    ) -> Result<()> {
        ctx.accounts.reentrancy_guard().start()?;
        let result = instructions::contribute_funds::handler(ctx, amount);
        ctx.accounts.reentrancy_guard().complete();
        result
    }

    pub fn close_fundraise(ctx: Context<CloseFundraise>) -> Result<()> {
        ctx.accounts.reentrancy_guard().start()?;
        let result = instructions::close_fundraise::handler(ctx);
        ctx.accounts.reentrancy_guard().complete();
        result
    }

    pub fn redeem_tokens(
        ctx: Context<RedeemTokens>,
        amount: u64
    ) -> Result<()> {
        ctx.accounts.reentrancy_guard().start()?;
        let result = instructions::redeem_tokens::handler(ctx, amount);
        ctx.accounts.reentrancy_guard().complete();
        result
    }
} 