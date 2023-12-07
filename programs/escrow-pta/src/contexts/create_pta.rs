use anchor_lang::prelude::*;
use anchor_spl::token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CreatePTA<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: we dont care what kind of account.
    pub owner: UncheckedAccount<'info>,
    #[account(seeds = [b"auth:", token_account.key().as_ref()], bump)]
    /// CHECKED: pda auth key for this pubkey and token mint.
    pub auth: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        seeds = [b"PTA:", owner.key().as_ref(), token_mint.key().as_ref()],
        bump,
        payer = payer,
        token::mint = token_mint,
        token::authority = auth
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePTA<'info> {
    pub fn create(&mut self, bumps: &CreatePTABumps) -> Result<()> {
        let accounts = Approve {
            authority: self.auth.to_account_info(),
            delegate: self.owner.to_account_info(),
            to: self.token_account.to_account_info(),
        };

        let owner = self.owner.key();
        let token_mint = self.token_mint.key();
        let auth_bump = bumps.auth.clone();

        let seeds = &[b"auth:", owner.as_ref(), token_mint.as_ref(), &[auth_bump]];

        let pda_signer = &[&seeds[..]];

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, pda_signer);

        approve(cpi_ctx, u64::MAX)
    }
}
