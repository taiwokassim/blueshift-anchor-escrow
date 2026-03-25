use anchor_lang::prelude::*;
mod state;
mod errors;
mod instructions;
use instructions::*; // brings in make, take, refund

//Program ID
declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    // Make instruction: creates escrow and locks Token A
    pub fn make(ctx: Context<instructions::make::Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)
    }

    // Take instruction: taker accepts swap, gets Token A, sends Token B
    pub fn take(ctx: Context<instructions::take::Take>) -> Result<()> {
        instructions::take::handler(ctx)
    }

    // Refund instruction: maker cancels escrow and gets Token A back
    pub fn refund(ctx: Context<instructions::refund::Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}
