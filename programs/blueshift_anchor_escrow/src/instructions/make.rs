use anchor_lang::prelude::*;
use crate::state::Escrow;
use crate::errors::EscrowError;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

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

    #[account(mut)]
    pub maker_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub maker_receive_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault_token_account.owner == escrow.key()
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    require!(amount > 0, EscrowError::InvalidAmount);

    // Validate ownership
    require!(
        ctx.accounts.maker_token_account.owner == ctx.accounts.maker.key(),
        EscrowError::InvalidMaker
    );

    let escrow = &mut ctx.accounts.escrow;
    escrow.seed = seed;
    escrow.maker = *ctx.accounts.maker.key;
    escrow.mint_a = ctx.accounts.maker_token_account.mint;
    escrow.mint_b = ctx.accounts.maker_receive_token_account.mint;
    escrow.receive = receive;
    escrow.bump = *ctx.bumps.get("escrow").unwrap();

    // Transfer Token A into vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.maker_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    Ok(())
}