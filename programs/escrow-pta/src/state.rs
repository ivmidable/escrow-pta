use anchor_lang::prelude::*;

#[account]
pub struct MAsk {
    pub open_asks: u64,
    pub nonce: u64,
    pub bump: u8,
    pub ta_bump: u8,
}

#[account]
pub struct TAsk {
    pub mint: Pubkey,
    pub that: u64,
    pub this: u64,
    pub bump: u8,
}

impl Space for MAsk {
    const INIT_SPACE: usize = 8 + 8 + 8 + 1 + 1;
}

impl Space for TAsk {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 1;
}
