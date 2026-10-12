use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub paused: bool,
    pub bump: u8,

    pub usdc_mint: Pubkey,
    pub usdc_vault: Pubkey,
    pub usdc_daily_limit: u64,
    pub usdc_window_start: i64,
    pub usdc_window_paid: u64,
    pub usdc_total_paid: u64,

    pub long_mint: Pubkey,
    pub long_vault: Pubkey,
    pub long_cap: u64,
    pub long_distributed: u64,
    pub long_daily_limit: u64,
    pub long_window_start: i64,
    pub long_window_paid: u64,
}
