use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("A6reKAfewwif4GxzqpYTr1CLMKp2mwytKaQrjWdPCsBi");

#[program]
pub mod cpi_example {
    use super::*;

    /// Initialize a new CPI example account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let cpi_example = &mut ctx.accounts.cpi_example;
        cpi_example.authority = ctx.accounts.authority.key();
        cpi_example.total_cpi_calls = 0;
        msg!("CPI Example initialized with authority: {}", cpi_example.authority);
        Ok(())
    }

    /// Demonstrate CPI to token program - transfer tokens
    pub fn transfer_tokens_via_cpi(
        ctx: Context<TransferTokensViaCpi>,
        amount: u64,
    ) -> Result<()> {
        let cpi_example = &mut ctx.accounts.cpi_example;
        
        // Create CPI context for calling the token program
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        
        // Call the token program's transfer function via CPI
        token::transfer(cpi_ctx, amount)?;
        
        // Update our own state
        cpi_example.total_cpi_calls = cpi_example.total_cpi_calls.checked_add(1).unwrap();
        
        msg!("Successfully transferred {} tokens via CPI! Total CPI calls: {}", 
             amount, cpi_example.total_cpi_calls);
        Ok(())
    }

    /// Demonstrate CPI to system program - transfer SOL
    pub fn transfer_sol_via_cpi(
        ctx: Context<TransferSolViaCpi>,
        amount: u64,
    ) -> Result<()> {
        let cpi_example = &mut ctx.accounts.cpi_example;
        
        // Create CPI context for calling the system program
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.from_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
            },
        );
        
        // Call the system program's transfer function via CPI
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;
        
        // Update our own state
        cpi_example.total_cpi_calls = cpi_example.total_cpi_calls.checked_add(1).unwrap();
        
        msg!("Successfully transferred {} lamports via CPI! Total CPI calls: {}", 
             amount, cpi_example.total_cpi_calls);
        Ok(())
    }

    /// Demonstrate multiple CPI calls in a single instruction
    pub fn multiple_cpi_calls(
        ctx: Context<MultipleCpiCalls>,
        token_amount: u64,
        sol_amount: u64,
    ) -> Result<()> {
        let cpi_example = &mut ctx.accounts.cpi_example;
        
        // First CPI: Transfer tokens
        let token_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::transfer(token_cpi_ctx, token_amount)?;
        
        // Second CPI: Transfer SOL
        let sol_cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.from_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(sol_cpi_ctx, sol_amount)?;
        
        // Update our own state
        cpi_example.total_cpi_calls = cpi_example.total_cpi_calls.checked_add(2).unwrap();
        
        msg!("Successfully completed multiple CPI calls! {} tokens transferred, {} lamports transferred. Total CPI calls: {}", 
             token_amount, sol_amount, cpi_example.total_cpi_calls);
        Ok(())
    }

    /// Demonstrate CPI with custom seeds and signers
    pub fn transfer_with_pda_authority(
        ctx: Context<TransferWithPdaAuthority>,
        amount: u64,
    ) -> Result<()> {
        let cpi_example = &mut ctx.accounts.cpi_example;
        
        // Create seeds for the PDA
        let authority_key = ctx.accounts.authority.key();
        let seeds = &[
            b"cpi_example",
            authority_key.as_ref(),
            &[ctx.bumps.pda_authority],
        ];
        let signer_seeds = &[&seeds[..]];
        
        // Create CPI context with PDA as signer
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.pda_authority.to_account_info(),
            },
            signer_seeds,
        );
        
        // Call the token program's transfer function via CPI with PDA authority
        token::transfer(cpi_ctx, amount)?;
        
        // Update our own state
        cpi_example.total_cpi_calls = cpi_example.total_cpi_calls.checked_add(1).unwrap();
        
        msg!("Successfully transferred {} tokens via CPI with PDA authority! Total CPI calls: {}", 
             amount, cpi_example.total_cpi_calls);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + CpiExample::INIT_SPACE
    )]
    pub cpi_example: Account<'info, CpiExample>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokensViaCpi<'info> {
    #[account(mut)]
    pub cpi_example: Account<'info, CpiExample>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferSolViaCpi<'info> {
    #[account(mut)]
    pub cpi_example: Account<'info, CpiExample>,
    #[account(mut)]
    pub from_account: SystemAccount<'info>,
    #[account(mut)]
    pub to_account: SystemAccount<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MultipleCpiCalls<'info> {
    #[account(mut)]
    pub cpi_example: Account<'info, CpiExample>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_account: SystemAccount<'info>,
    #[account(mut)]
    pub to_account: SystemAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferWithPdaAuthority<'info> {
    #[account(mut)]
    pub cpi_example: Account<'info, CpiExample>,
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is a PDA that will be used as authority
    #[account(
        seeds = [b"cpi_example", authority.key().as_ref()],
        bump
    )]
    pub pda_authority: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct CpiExample {
    pub authority: Pubkey,
    pub total_cpi_calls: u64,
}