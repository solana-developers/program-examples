use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{close_account, CloseAccount},
    token_interface::{
        spl_pod::optional_keys::OptionalNonZeroPubkey,
        spl_token_2022::{
            extension::{
                mint_close_authority::MintCloseAuthority, BaseStateWithExtensions,
                StateWithExtensions,
            },
            state::Mint as MintState,
        },
        Mint, Token2022,
    },
};
declare_id!("AcfQLsYKuzprcCNH1n96pKKgAbAnZchwpbr3gbVN742n");

#[program]
pub mod mint_close_authority {
    use super::*;

    pub fn initialize(mut context: Context<InitializeAccountConstraints>) -> Result<()> {
        handle_check_mint_data(&mut context.accounts)?;
        Ok(())
    }

    pub fn close(context: Context<CloseAccountConstraints>) -> Result<()> {
        // cpi to token extensions programs to close mint account
        // alternatively, this can also be done in the client
        close_account(CpiContext::new(
            context.accounts.token_program.key(),
            CloseAccount {
                account: context.accounts.mint_account.to_account_info(),
                destination: context.accounts.authority.to_account_info(),
                authority: context.accounts.authority.to_account_info(),
            },
        ))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccountConstraints<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 2,
        mint::authority = payer,
        extensions::close_authority::authority = payer,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

// helper to check mint data, and demonstrate how to read mint extension data within a program
pub fn handle_check_mint_data(accounts: &mut InitializeAccountConstraints) -> Result<()> {
        let mint = &accounts.mint_account.to_account_info();
        let mint_data = mint.data.borrow();
        let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
        let extension_data = mint_with_extension.get_extension::<MintCloseAuthority>()?;

        assert_eq!(
            extension_data.close_authority,
            OptionalNonZeroPubkey::try_from(Some(accounts.payer.key()))?
        );

        msg!("{:?}", extension_data);
        Ok(())
    }


#[derive(Accounts)]
pub struct CloseAccountConstraints<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        extensions::close_authority::authority = authority,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
}
