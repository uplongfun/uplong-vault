use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::*,
    errors::VaultError,
    events::Distributed,
    state::{Config, Recipient},
    utils::{take_from_window, transfer_from_vault},
};

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = operator @ VaultError::NotOperator,
        constraint = config.long_vault == vault.key() @ VaultError::WrongVault,
        constraint = config.long_mint == mint.key() @ VaultError::WrongMint,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: the trader receiving $LONG; only their address is used.
    pub user: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = operator,
        space = 8 + Recipient::INIT_SPACE,
        seeds = [RECIPIENT_SEED, user.key().as_ref()],
        bump,
    )]
    pub recipient: Account<'info, Recipient>,
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

impl Distribute<'_> {
    pub fn distribute(&mut self, earned: u64) -> Result<()> {
        require!(!self.config.paused, VaultError::Paused);
        if self.recipient.user == Pubkey::default() {
            self.recipient.user = self.user.key();
        }
        let amount = earned.saturating_sub(self.recipient.distributed);
        require!(amount > 0, VaultError::NothingToPay);
        let total = self
            .config
            .long_distributed
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;
        require!(total <= self.config.long_cap, VaultError::CapReached);
        let now = Clock::get()?.unix_timestamp;
        let (window_start, window) = take_from_window(
            self.config.long_window_start,
            self.config.long_window_paid,
            self.config.long_daily_limit,
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
        config.long_window_start = window_start;
        config.long_window_paid = window;
        config.long_distributed = total;
        self.recipient.distributed = earned;
        emit!(Distributed {
            user: self.recipient.user,
            amount,
            total_to_user: earned,
            total_distributed: total,
        });
        Ok(())
    }
}
