use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    pub seed: u64,        // allows multiple escrows per maker
    pub maker: Pubkey,    // creator of the escrow
    pub mint_a: Pubkey,   // token A (what maker gives)
    pub mint_b: Pubkey,   // token B (what maker wants)
    pub receive: u64,     // amount of token B expected
    pub bump: u8,         // PDA bump
}
