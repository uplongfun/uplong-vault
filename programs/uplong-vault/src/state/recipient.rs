use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Recipient {
    pub user: Pubkey,
    pub distributed: u64,
}
