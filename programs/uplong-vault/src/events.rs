use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub admin: Pubkey,
    pub operator: Pubkey,
}

#[event]
pub struct UsdcSet {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub daily_limit: u64,
}

#[event]
pub struct LongSet {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub cap: u64,
    pub daily_limit: u64,
}

#[event]
pub struct Paid {
    pub id: [u8; 32],
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Distributed {
    pub user: Pubkey,
    pub amount: u64,
    pub total_to_user: u64,
    pub total_distributed: u64,
}

#[event]
pub struct OperatorSet {
    pub operator: Pubkey,
}

#[event]
pub struct PausedSet {
    pub paused: bool,
}

#[event]
pub struct DailyLimitSet {
    pub usdc: bool,
    pub daily_limit: u64,
}

#[event]
pub struct AdminSet {
    pub admin: Pubkey,
}
