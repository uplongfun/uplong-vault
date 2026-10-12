use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::*,
    errors::VaultError,
    events::Paid,
    state::{Config, Receipt},
    utils::{take_from_window, transfer_from_vault},
};

#[derive(Accounts)]
#[instruction(id: [u8; 32])]
pub struct Pay<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = operator @ VaultError::NotOperator,
        constraint = config.usdc_vault == vault.key() @ VaultError::WrongVault,
        constraint = config.usdc_mint == mint.key() @ VaultError::WrongMint,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: the trader receiving USDC; only their address is used.
    pub user: UncheckedAccount<'info>,
    #[account(
        init,
        payer = operator,
        space = 8 + Receipt::INIT_SPACE,
        seeds = [RECEIPT_SEED, id.as_ref()],
        bump,
    )]
    pub receipt: Account<'info, Receipt>,
    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl Pay<'_> {
    pub fn pay(&mut self, id: [u8; 32], amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::ZeroAmount);
        require!(!self.config.paused, VaultError::Paused);
        let now = Clock::get()?.unix_timestamp;
        let (window_start, window) = take_from_window(
            self.config.usdc_window_start,
            self.config.usdc_window_paid,
            self.config.usdc_daily_limit,
            amount,
            now,
        )?;
        transfer_from_vault(
            &self.config,
            &self.token_program,
            &self.vault,
            &self.mint,
            &self.user_token,
            amount,
        )?;
        let config = &mut self.config;
        config.usdc_window_start = window_start;
        config.usdc_window_paid = window;
        config.usdc_total_paid = config
            .usdc_total_paid
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        let receipt = &mut self.receipt;
        receipt.id = id;
        receipt.user = self.user.key();
        receipt.amount = amount;
        receipt.paid_at = now;
        emit!(Paid {
            id,
            user: receipt.user,
            amount,
        });
        Ok(())
    }
}
