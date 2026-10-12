//! uplong vault
//! Website: https://uplong.fun
//! Twitter: https://x.com/upLongFun

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("LongtNv8Jfv4EBVwRfoU67iaT9vcD2Sk1FF3XYi2zSt");

#[program]
pub mod uplong_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, operator: Pubkey) -> Result<()> {
        ctx.accounts.initialize(operator, ctx.bumps.config)
    }

    pub fn init_usdc(ctx: Context<InitUsdc>, daily_limit: u64) -> Result<()> {
        ctx.accounts.init_usdc(daily_limit)
    }

    pub fn init_long(ctx: Context<InitLong>, cap: u64, daily_limit: u64) -> Result<()> {
        ctx.accounts.init_long(cap, daily_limit)
    }

    pub fn pay(ctx: Context<Pay>, id: [u8; 32], amount: u64) -> Result<()> {
        ctx.accounts.pay(id, amount)
    }

    pub fn distribute(ctx: Context<Distribute>, earned: u64) -> Result<()> {
        ctx.accounts.distribute(earned)
    }

    pub fn set_operator(ctx: Context<Admin>, operator: Pubkey) -> Result<()> {
        ctx.accounts.set_operator(operator)
    }

    pub fn set_paused(ctx: Context<Admin>, paused: bool) -> Result<()> {
        ctx.accounts.set_paused(paused)
    }

    pub fn set_usdc_daily_limit(ctx: Context<Admin>, daily_limit: u64) -> Result<()> {
        ctx.accounts.set_usdc_daily_limit(daily_limit)
    }

    pub fn set_long_daily_limit(ctx: Context<Admin>, daily_limit: u64) -> Result<()> {
        ctx.accounts.set_long_daily_limit(daily_limit)
    }

    pub fn set_admin(ctx: Context<Admin>, admin: Pubkey) -> Result<()> {
        ctx.accounts.set_admin(admin)
    }
}
