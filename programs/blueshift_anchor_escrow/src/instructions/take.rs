use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", &escrow.seed.to_le_bytes()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: validated via escrow
    #[account(mut, address = escrow.maker)]
    pub maker: AccountInfo<'info>,

    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        constraint = taker_token_account_a.owner == taker.key(),
        constraint = taker_token_account_a.mint == escrow.mint_a
    )]
    pub taker_token_account_a: Account<'info, TokenAccount>, // receives A

    #[account(
        mut,
        constraint = taker_token_account_b.owner == taker.key(),
        constraint = taker_token_account_b.mint == escrow.mint_b
    )]
    pub taker_token_account_b: Account<'info, TokenAccount>, // sends B

    #[account(
        mut,
        constraint = vault_token_account.owner == escrow.key(),
        constraint = vault_token_account.mint == escrow.mint_a
    )]
    pub vault_token_account: Account<'info, TokenAccount>, // holds A

    #[account(
        mut,
        constraint = maker_token_account.owner == maker.key(),
        constraint = maker_token_account.mint == escrow.mint_b
    )]
    pub maker_token_account: Account<'info, TokenAccount>, // maker receives B

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Take>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;

    // 🔐 PDA signer
    let seeds = &[
        b"escrow",
        &escrow.seed.to_le_bytes(),
        &[escrow.bump],
    ];
    let signer = &[&seeds[..]];

    // ✅ 1. Transfer Token B (taker → maker)
    let transfer_b = Transfer {
        from: ctx.accounts.taker_token_account_b.to_account_info(),
        to: ctx.accounts.maker_token_account.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_b,
        ),
        escrow.receive,
    )?;

    // ✅ 2. Transfer Token A (vault → taker)
    let transfer_a = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_a,
            signer,
        ),
        ctx.accounts.vault_token_account.amount,
    )?;

    // ✅ 3. Close vault (send rent to maker)
    let close = CloseAccount {
        account: ctx.accounts.vault_token_account.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };

    token::close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close,
            signer,
        ),
    )?;

    Ok(())
}
