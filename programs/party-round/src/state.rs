use anchor_lang::prelude::*;

pub const MAX_OWNERS: usize = 10;

#[account]
pub struct DaoState {
    pub token_name: String,
    pub token_symbol: String,
    pub total_supply: u64,
    pub fundraise_end_ts: i64,
    pub token_price_lamports: u64,
    pub allowlisted_addresses: Vec<Pubkey>,
    pub fundraise_ended: bool,
    pub total_contributions: u64,
    pub total_contributors: u32,
    pub liquidity_locked: bool,
    pub lock_end_ts: i64,
}

#[account]
pub struct MultisigConfig {
    pub owners: Vec<Pubkey>,     // Array of owner addresses
    pub threshold: u8,           // Required signature count
    pub owner_set_seqno: u64,    // Increments on owner changes
}

impl DaoState {
    pub const MAX_SIZE: usize = 
        4 + 32 +  // token_name (max length)
        4 + 32 +  // token_symbol
        8 +       // total_supply
        8 +       // fundraise_end_ts
        8 +       // token_price_lamports
        4 + (32 * 100) + // allowlisted_addresses (up to 100 addresses)
        1 +       // fundraise_ended
        8 +       // total_contributions
        4 +       // total_contributors
        1 +       // liquidity_locked
        8;        // lock_end_ts
}

impl MultisigConfig {
    pub const MAX_SIZE: usize = 
        4 + (MAX_OWNERS * 32) + // owners array
        1 +                     // threshold
        8;                      // owner_set_seqno
} 