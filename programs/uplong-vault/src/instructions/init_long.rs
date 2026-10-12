use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::*, errors::VaultError, events::LongSet, state::Config};

#[derive(Accounts)]
pub struct InitLong<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = admin @ VaultError::NotAdmin)]
    pub config: Account<'info, Config>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [LONG_VAULT_SEED],
        bump,
        token::mint = mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl InitLong<'_> {
    pub fn init_long(&mut self, cap: u64, daily_limit: u64) -> Result<()> {
        let config = &mut self.config;
        require!(
            config.long_mint == Pubkey::default(),
            VaultError::AlreadySet
        );
        config.long_mint = self.mint.key();
        config.long_vault = self.vault.key();
        config.long_cap = cap;
        config.long_daily_limit = daily_limit;
        config.long_window_start = Clock::get()?.unix_timestamp;
        emit!(LongSet {
            mint: config.long_mint,
            vault: config.long_vault,
            cap,
            daily_limit,
        });
        Ok(())
    }
}
