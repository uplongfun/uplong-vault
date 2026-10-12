use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("amount must be positive")]
    ZeroAmount,
    #[msg("payouts are paused")]
    Paused,
    #[msg("the 24-hour payout limit would be exceeded")]
    DailyLimitReached,
    #[msg("this user has already been paid up to that total")]
    NothingToPay,
    #[msg("the $LONG pool's total cap would be exceeded")]
    CapReached,
    #[msg("this mint is already set")]
    AlreadySet,
    #[msg("wrong vault account")]
    WrongVault,
    #[msg("wrong mint")]
    WrongMint,
    #[msg("only the operator can do this")]
    NotOperator,
    #[msg("only the admin can do this")]
    NotAdmin,
    #[msg("arithmetic overflow")]
    Overflow,
}
