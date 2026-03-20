use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        has_one = maker,
        close = maker,
        seeds = [b"escrow", &escrow.seed.to_le_bytes()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        constraint = vault_token_account.owner == escrow.key(),
        constraint = vault_token_account.mint == escrow.mint_a
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = maker_token_account.owner == maker.key(),
        constraint = maker_token_account.mint == escrow.mint_a
    )]
    pub maker_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // ✅ PDA signer seeds
    let seeds = &[
        b"escrow",
        &escrow.seed.to_le_bytes(),
        &[escrow.bump],
    ];
    let signer = &[&seeds[..]];

    // ✅ Transfer ALL Token A back to maker
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.maker_token_account.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        ),
        ctx.accounts.vault_token_account.amount,
    )?;

    // ✅ Close vault account (send rent back to maker)
    let close_accounts = CloseAccount {
        account: ctx.accounts.vault_token_account.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    token::close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close_accounts,
            signer,
        ),
    )?;

    Ok(())
}
