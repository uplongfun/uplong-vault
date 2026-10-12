use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::{constants::*, errors::VaultError, state::Config};

pub fn take_from_window(
    start: i64,
    paid: u64,
    limit: u64,
    amount: u64,
    now: i64,
) -> Result<(i64, u64)> {
    let (start, paid) = if now - start >= DAY {
        (now, 0)
    } else {
        (start, paid)
    };
    let window = paid.checked_add(amount).ok_or(VaultError::Overflow)?;
    require!(window <= limit, VaultError::DailyLimitReached);
    Ok((start, window))
}

pub fn transfer_from_vault<'info>(
    config: &Account<'info, Config>,
    token_program: &Interface<'info, TokenInterface>,
    vault: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let seeds: &[&[u8]] = &[CONFIG_SEED, &[config.bump]];
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: vault.to_account_info(),
                mint: mint.to_account_info(),
                to: to.to_account_info(),
                authority: config.to_account_info(),
            },
            &[seeds],
        ),
        amount,
        mint.decimals,
    )
}
