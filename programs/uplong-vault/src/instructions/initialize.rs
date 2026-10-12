use anchor_lang::prelude::*;

use crate::{constants::*, events::Initialized, state::Config};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn initialize(&mut self, operator: Pubkey, bump: u8) -> Result<()> {
        let config = &mut self.config;
        config.admin = self.admin.key();
        config.operator = operator;
        config.paused = false;
        config.bump = bump;
        emit!(Initialized {
            admin: config.admin,
            operator,
        });
        Ok(())
    }
}
