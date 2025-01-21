use anchor_lang::prelude::*;

#[error_code]
pub enum DaoError {
    #[msg("Fundraise period has ended")]
    FundraiseEnded,
    
    #[msg("Address not in allowlist")]
    NotAllowlisted,
    
    #[msg("Fundraise period has not ended yet")]
    FundraiseNotEnded,
    
    #[msg("Invalid contribution amount")]
    InvalidContributionAmount,
    
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    
    #[msg("Treasury has insufficient funds")]
    InsufficientTreasuryFunds,
    
    #[msg("Invalid redemption amount")]
    InvalidRedemptionAmount,
    
    #[msg("Address not in DAO allowlist")]
    AddressNotAllowed,
    
    #[msg("Liquidity is already locked")]
    LiquidityAlreadyLocked,
    
    #[msg("Liquidity not locked")]
    LiquidityNotLocked,
    
    #[msg("Liquidity is still locked")]
    LiquidityStillLocked,

    #[msg("Unauthorized multisig owner")]
    Unauthorized,

    #[msg("Threshold exceeds number of owners")]
    InvalidThreshold,

    #[msg("Invalid number of owners")]
    InvalidOwnerCount,

    #[msg("Duplicate owner address")]
    DuplicateOwner,

    #[msg("Reentrancy detected")]
    ReentrancyAttempt,
} 