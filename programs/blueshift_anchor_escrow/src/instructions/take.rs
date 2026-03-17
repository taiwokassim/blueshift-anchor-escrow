use anchor_lang::prelude::*;
use crate::state::Escrow;
use crate::errors::EscrowError;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut, has_one = mint_a, has_one = mint_b)]
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
    pub maker: AccountInfo<'info>, // maker to receive Token B

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Take>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // Transfer Token B from taker to maker
    let cpi_accounts_b = Transfer {
        from: ctx.accounts.taker_token_account_b.to_account_info(),
        to: ctx.accounts.maker.clone(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program.clone(), cpi_accounts_b), escrow.receive)?;

    // Transfer Token A from vault to taker
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    // Escrow PDA is the authority
    let seeds = &[b"escrow", &escrow.seed.to_le_bytes(), &[escrow.bump]];
    let signer = &[&seeds[..]];
    token::transfer(CpiContext::new_with_signer(cpi_program, cpi_accounts_a, signer), ctx.accounts.vault_token_account.amount)?;

    // Close vault and escrow account
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.vault_token_account.to_account_info(),
            destination: ctx.accounts.maker.clone(),
            authority: ctx.accounts.escrow.to_account_info(),
        },
        signer,
    ))?;

    Ok(())
}