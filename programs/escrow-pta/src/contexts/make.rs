use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::state::*;

impl<'info> Make<'info> {
    pub fn make(&mut self, this: u64, that: u64) -> Result<()> {
        self.mask.nonce += 1;
        self.mask.open_asks += 1;
        self.task.this = this;
        self.task.that = that;
        self.task.mint = self.token_mint.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init_if_needed, seeds = [b"MAsk:", token_account.key().as_ref()], bump, payer = payer, space = MAsk::INIT_SPACE)]
    pub mask: Account<'info, MAsk>,
    #[account(init, seeds = [b"TAsk:", token_account.key().as_ref(), mask.nonce.to_le_bytes().as_ref()], bump, payer = payer, space = TAsk::INIT_SPACE)]
    pub task: Account<'info, TAsk>,
    #[account(seeds = [b"auth:", token_account.key().as_ref()], bump)]
    /// CHECKED: pda auth key for this pubkey and token mint.
    pub auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"PTA:", payer.key().as_ref(), token_mint.key().as_ref()],
        bump,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}
