use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, MintTo};  // Import necessary types and modules
use spl_program::option::COption; // Import COption from spl-program crate

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod aleph_andromeda_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_authority: Pubkey) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.mint_authority = COption::Some(mint_authority);  // Correct use of COption
        Ok(())
    }

    pub fn mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let authority = &ctx.accounts.authority;
        
        // Check if the authority is allowed to mint
        if authority.key != &mint.mint_authority.unwrap() {
            return Err(ErrorCode::UnauthorizedMint.into());
        }

        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, mint::decimals = 9, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,  // Add the token program here
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(signer)]
    pub authority: Signer<'info>,  // Authority must be of type Signer
    pub token_program: Program<'info, Token>,
}

// Custom errors
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized minting attempt.")]
    UnauthorizedMint,
}
