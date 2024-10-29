use {
    crate::{state::*, SteelInstruction},
    steel::*,
};

instruction!(SteelInstruction, TakeOffer);

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TakeOffer {}

impl TakeOffer {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [offer_info, token_mint_a, token_mint_b, maker_token_account_b, taker_token_account_a, taker_token_account_b, vault, maker, taker, payer, token_program, associated_token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        maker.is_writable()?;
        system_program.is_program(&system_program::ID)?;

        let offer = offer_info.as_account_mut::<Offer>(program_id)?;

        if taker_token_account_a.lamports() == 0 {
            create_associated_token_account(
                payer,
                taker,
                taker_token_account_a,
                token_mint_a,
                system_program,
                token_program,
                associated_token_program,
            )?;
        }

        if maker_token_account_b.lamports() == 0 {
            create_associated_token_account(
                payer,
                maker,
                maker_token_account_b,
                token_mint_b,
                system_program,
                token_program,
                associated_token_program,
            )?;
        }

        solana_program::msg!("Vault A Balance Before Transfer: {}", vault.as_token_account()?.amount);
        solana_program::msg!("Taker A Balance Before Transfer: {}", taker_token_account_a.as_token_account()?.amount);
        solana_program::msg!("Maker B Balance Before Transfer: {}", maker_token_account_b.as_token_account()?.amount);
        solana_program::msg!("Taker B Balance Before Transfer: {}", taker_token_account_b.as_token_account()?.amount);
        
        transfer(
            taker,
            taker_token_account_b,
            maker_token_account_b,
            token_program,
            offer.token_b_wanted_amount,
        )?;

        
        transfer_signed(
            offer_info,
            vault,
            taker_token_account_a,
            token_program,
            vault.as_token_account()?.amount,
            &[
                b"offer",
                maker.key.as_ref(),
                offer.id.to_be_bytes().as_ref(),
            ],
        )?;

        solana_program::msg!("Vault A Balance After Transfer: {}", vault.as_token_account()?.amount);
        solana_program::msg!("Taker A Balance After Transfer: {}", taker_token_account_a.as_token_account()?.amount);
        solana_program::msg!("Maker B Balance After Transfer: {}", maker_token_account_b.as_token_account()?.amount);
        solana_program::msg!("Taker B Balance After Transfer: {}", taker_token_account_b.as_token_account()?.amount);

        invoke_signed_with_bump(
            &spl_token::instruction::close_account(
                token_program.key,
                vault.key,
                taker.key,
                offer_info.key,
                &[],
            )?,
            &[
                vault.clone(),
                taker.clone(),
                offer_info.clone(),
            ],
            &[
                b"offer",
                maker.key.as_ref(),
                offer.id.to_be_bytes().as_ref(),
            ],
            offer.bump,
        )?;

        offer_info.close(taker)?;

        Ok(())
    }
}
