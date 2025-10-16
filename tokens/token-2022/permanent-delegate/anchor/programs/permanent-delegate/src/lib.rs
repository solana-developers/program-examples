use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::extension::permanent_delegate::PermanentDelegate,
    token_interface::{
        spl_pod::optional_keys::OptionalNonZeroPubkey,
        spl_token_2022::{
            extension::{BaseStateWithExtensions, StateWithExtensions},
            state::Mint as MintState,
        },
        Mint, Token2022,
    },
};

declare_id!("A9rxKS84ZoJVyeTfQbCEfxME2vvAM4uwSMjkmhR5XWb1");

#[program]
pub mod permanent_delegate {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.check_mint_data()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 2,
        mint::authority = payer,
        extensions::permanent_delegate::delegate = payer,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

// helper to check mint data, and demonstrate how to read mint extension data within a program
impl<'info> Initialize<'info> {
    pub fn check_mint_data(&self) -> Result<()> {
        let mint = &self.mint_account.to_account_info();
        let mint_data = mint.data.borrow();
        let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
        let extension_data = mint_with_extension.get_extension::<PermanentDelegate>()?;

        assert_eq!(
            extension_data.delegate,
            OptionalNonZeroPubkey::try_from(Some(self.payer.key()))?
        );

        msg!("{:?}", extension_data);
        Ok(())
    }
}
