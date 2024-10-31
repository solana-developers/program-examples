use {
    crate::{state::*, EscrowInstruction},
    steel::*,
};

instruction!(EscrowInstruction, TakeOffer);
//  TakeOffer Instruction
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TakeOffer {}

impl TakeOffer {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [
            // accounts order
            offer_info,
            token_mint_a,
            token_mint_b,
            maker_token_account_b,
            taker_token_account_a,
            taker_token_account_b,
            vault,
            maker,
            taker,
            payer,
            token_program,
            associated_token_program,
            system_program
        ] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Ensure the taker is a signer
        //
        taker.is_signer()?;

        // Validate the offer
        //
        let offer = offer_info
            .as_account::<Offer>(program_id)?
            .assert(|offer| {
                offer.maker == *maker.key
                    && offer.token_mint_a == *token_mint_a.key
                    && offer.token_mint_b == *token_mint_b.key
            })?;

        let offer_seeds = &[Offer::SEEDS, maker.key.as_ref(), &offer.id.to_le_bytes()];

        // validate offer account
        //
        offer_info.has_seeds(offer_seeds, program_id)?;

        // Create taker token a account, if needed
        //
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

        // Create maker token b account, if needed
        //
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

        // Validate the token accounts, then get the current amounts
        //
        let vault_amount = vault
            .as_associated_token_account(offer_info.key, token_mint_a.key)?
            .amount;
        let taker_a_amount_before_transfer = taker_token_account_a
            .as_associated_token_account(taker.key, token_mint_a.key)?
            .amount;
        let maker_b_amount_before_transfer = maker_token_account_b
            .as_associated_token_account(maker.key, token_mint_b.key)?
            .amount;
        let taker_b_amount_before_transfer = taker_token_account_b
            .as_associated_token_account(taker.key, token_mint_b.key)?
            .amount;

        solana_program::msg!("Vault A Balance Before Transfer: {}", vault_amount);
        solana_program::msg!(
            "Taker A Balance Before Transfer: {}",
            taker_a_amount_before_transfer
        );
        solana_program::msg!(
            "Maker B Balance Before Transfer: {}",
            maker_b_amount_before_transfer
        );
        solana_program::msg!(
            "Taker B Balance Before Transfer: {}",
            taker_b_amount_before_transfer
        );

        // taker makes a transfer of the wanted amount to maker
        //
        transfer(
            taker,
            taker_token_account_b,
            maker_token_account_b,
            token_program,
            offer.token_b_wanted_amount, // offer amount
        )?;

        // tokens in the vault is transfered to the taker
        //
        transfer_signed(
            offer_info,
            vault,
            taker_token_account_a,
            token_program,
            vault_amount, // all tokens in the vault
            offer_seeds,
        )?;

        let taker_a_amount = taker_token_account_a.as_token_account()?.amount;
        let maker_b_amount = maker_token_account_b.as_token_account()?.amount;

        // assert the token balances after transfers
        //
        assert_eq!(
            taker_a_amount,
            vault_amount + taker_a_amount_before_transfer
        );
        assert_eq!(
            maker_b_amount,
            maker_b_amount_before_transfer + offer.token_b_wanted_amount
        );

        let vault_amount = vault.as_token_account()?.amount;
        let taker_b_amount = taker_token_account_b.as_token_account()?.amount;

        solana_program::msg!("Vault A Balance After Transfer: {}", vault_amount);
        solana_program::msg!("Taker A Balance After Transfer: {}", taker_a_amount);
        solana_program::msg!("Maker B Balance After Transfer: {}", maker_b_amount);
        solana_program::msg!("Taker B Balance After Transfer: {}", taker_b_amount);

        // close the vault because it is no longer needed
        //
        invoke_signed_with_bump(
            &spl_token::instruction::close_account(
                token_program.key, // token program
                vault.key, // token account to close
                taker.key, // account to transfer lamports
                offer_info.key, // token account ownder
                &[offer_info.key], // signer pubkeys
            )?,
            &[vault.clone(), taker.clone(), offer_info.clone()],
            offer_seeds,
            offer.bump,
        )?;

        // close the offer account
        //
        offer_info.close(taker)?;

        Ok(())
    }
}
