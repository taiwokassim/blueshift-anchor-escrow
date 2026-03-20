use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

use crate::state::Escrow;
use crate::errors::EscrowError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", &seed.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        constraint = maker_token_account.owner == maker.key(),
    )]
    pub maker_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = maker_receive_token_account.owner == maker.key(),
    )]
    pub maker_receive_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        token::mint = mint_a,
        token::authority = escrow
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Make>,
    seed: u64,
    receive: u64,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, EscrowError::InvalidAmount);

    let escrow = &mut ctx.accounts.escrow;

    // ✅ Save escrow state
    escrow.seed = seed;
    escrow.maker = ctx.accounts.maker.key();
    escrow.mint_a = ctx.accounts.maker_token_account.mint;
    escrow.mint_b = ctx.accounts.maker_receive_token_account.mint;
    escrow.receive = receive;
    escrow.bump = ctx.bumps.escrow;

    // ✅ Transfer Token A → Vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.maker_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
        amount,
    )?;

    Ok(())
}
