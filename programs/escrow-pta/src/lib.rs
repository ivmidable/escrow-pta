use anchor_lang::prelude::*;

declare_id!("FMQmVyXezRLnmWbiuuMyRm5yd9D4tSnta3FDfKyExhhS");

pub mod contexts;
pub mod state;

use contexts::*;

#[program]
pub mod escrow_pta {

    use super::*;

    pub fn create_pta(ctx: Context<CreatePTA>) -> Result<()> {
        ctx.accounts.create(&ctx.bumps)
    }

    pub fn make(ctx: Context<Make>, this: u64, that: u64) -> Result<()> {
        ctx.accounts.make(this, that)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take(&ctx.bumps)
    }
}
