use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Receipt {
    pub id: [u8; 32],
    pub user: Pubkey,
    pub amount: u64,
    pub paid_at: i64,
}
