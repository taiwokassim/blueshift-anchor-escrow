use anchor_lang::prelude::*;
use crate::state::Escrow;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub taker_token_account_a: Account<'info, TokenAccount>, // receives Token A

    #[account(mut)]
    pub taker_token_account_b: Account<'info, TokenAccount>, // sends Token B

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>, // holds Token A

    #[account(mut)]
    pub maker_token_account: Account<'info, TokenAccount>, // maker receives Token B

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Take>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // ✅ Transfer Token B (taker → maker)
    let cpi_accounts_b = Transfer {
        from: ctx.accounts.taker_token_account_b.to_account_info(),
        to: ctx.accounts.maker_token_account.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(
        CpiContext::new(cpi_program.clone(), cpi_accounts_b),
        escrow.receive,
    )?;

    // ✅ Transfer Token A (vault → taker)
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    let seeds = &[b"escrow", &escrow.seed.to_le_bytes(), &[escrow.bump]];
    let signer = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts_a, signer),
        ctx.accounts.vault_token_account.amount,
    )?;

    // ✅ Close vault account
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.vault_token_account.to_account_info(),
            destination: ctx.accounts.taker.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        },
        signer,
    ))?;

    Ok(())
}
