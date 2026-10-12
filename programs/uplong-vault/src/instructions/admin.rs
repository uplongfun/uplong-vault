use anchor_lang::prelude::*;

use crate::{
    constants::*,
    errors::VaultError,
    events::{AdminSet, DailyLimitSet, OperatorSet, PausedSet},
    state::Config,
};

#[derive(Accounts)]
pub struct Admin<'info> {
    pub admin: Signer<'info>,
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = admin @ VaultError::NotAdmin)]
    pub config: Account<'info, Config>,
}

impl Admin<'_> {
    pub fn set_operator(&mut self, operator: Pubkey) -> Result<()> {
        self.config.operator = operator;
        emit!(OperatorSet { operator });
        Ok(())
    }

    pub fn set_paused(&mut self, paused: bool) -> Result<()> {
        self.config.paused = paused;
        emit!(PausedSet { paused });
        Ok(())
    }

    pub fn set_usdc_daily_limit(&mut self, daily_limit: u64) -> Result<()> {
        self.config.usdc_daily_limit = daily_limit;
        emit!(DailyLimitSet {
            usdc: true,
            daily_limit
        });
        Ok(())
    }

    pub fn set_long_daily_limit(&mut self, daily_limit: u64) -> Result<()> {
        self.config.long_daily_limit = daily_limit;
        emit!(DailyLimitSet {
            usdc: false,
            daily_limit
        });
        Ok(())
    }

    pub fn set_admin(&mut self, admin: Pubkey) -> Result<()> {
        self.config.admin = admin;
        emit!(AdminSet { admin });
        Ok(())
    }
}
