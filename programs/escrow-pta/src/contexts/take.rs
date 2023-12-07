use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::state::*;

impl<'info> Take<'info> {
    pub fn take(&mut self, bumps: &TakeBumps) -> Result<()> {
        self.mask.open_asks -= 1;

        //check to see if amount is less than ask
        if self.maker_this_ta.amount < self.task.this {
            //close task account and give taker the rent
            return Ok(());
        }

        // ####### THIS ###########
        let accounts = TransferChecked {
            from: self.maker_this_ta.to_account_info(),
            to: self.taker_this_ta.to_account_info(),
            mint: self.this_mint.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let maker = self.maker.key();
        let maker_this_ta = self.maker_this_ta.key();
        let auth_bump = bumps.auth.clone();
        let seeds = &[
            b"auth:",
            maker.as_ref(),
            maker_this_ta.as_ref(),
            &[auth_bump],
        ];

        let pda_signer = &[&seeds[..]];

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, pda_signer);

        transfer_checked(cpi_ctx, self.task.this, self.this_mint.decimals)?;

        // ######### THAT ##########

        let accounts = TransferChecked {
            from: self.taker_that_ta.to_account_info(),
            to: self.maker_that_ta.to_account_info(),
            mint: self.that_mint.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(cpi_ctx, self.task.that, self.that_mint.decimals)
    }
}

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: we are just using this to build the seeds for PDA.
    pub maker: UncheckedAccount<'info>,
    #[account(seeds = [b"auth:", maker_this_ta.key().as_ref()], bump)]
    /// CHECKED: pda auth key for this pubkey and token mint.
    pub auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"MAsk"], bump = mask.bump)]
    pub mask: Account<'info, MAsk>,
    #[account(mut, seeds = [b"TAsk", maker_this_ta.key().as_ref(), mask.nonce.to_le_bytes().as_ref()], bump=task.bump, close = taker)]
    pub task: Account<'info, TAsk>,
    #[account(mut, seeds = [b"PTA:", maker.key().as_ref(), task.mint.key().as_ref()], bump=mask.ta_bump)]
    pub maker_this_ta: InterfaceAccount<'info, TokenAccount>,
    pub taker_this_ta: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = maker)]
    pub maker_that_ta: InterfaceAccount<'info, TokenAccount>,
    pub taker_that_ta: InterfaceAccount<'info, TokenAccount>,
    pub this_mint: InterfaceAccount<'info, Mint>,
    pub that_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}
