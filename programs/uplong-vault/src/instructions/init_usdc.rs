use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::*, errors::VaultError, events::UsdcSet, state::Config};

#[derive(Accounts)]
pub struct InitUsdc<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = admin @ VaultError::NotAdmin)]
    pub config: Account<'info, Config>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [USDC_VAULT_SEED],
        bump,
        token::mint = mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl InitUsdc<'_> {
    pub fn init_usdc(&mut self, daily_limit: u64) -> Result<()> {
        let config = &mut self.config;
        require!(
            config.usdc_mint == Pubkey::default(),
            VaultError::AlreadySet
        );
        config.usdc_mint = self.mint.key();
        config.usdc_vault = self.vault.key();
        config.usdc_daily_limit = daily_limit;
        config.usdc_window_start = Clock::get()?.unix_timestamp;
        emit!(UsdcSet {
            mint: config.usdc_mint,
            vault: config.usdc_vault,
            daily_limit,
        });
        Ok(())
    }
}
