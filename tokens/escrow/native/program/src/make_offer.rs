use {
    crate::state::*,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_associated_token_account::instruction as associated_token_account_instruction,
    spl_token::{instruction as token_instruction, state::Account as TokenAccount},
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MakeOffer {
    pub id: u64,
    pub token_a_offered_amount: u64,
    pub token_b_wanted_amount: u64,
}

impl MakeOffer {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        args: MakeOffer,
    ) -> ProgramResult {
        let [offer, token_mint_a, token_mint_b, maker_token_account_a, vault, maker, payer, token_program, associated_token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        solana_program::msg!("here right now!");

        let (_, bump) = Pubkey::find_program_address(
            &[b"offer", maker.key.as_ref(), args.id.to_be_bytes().as_ref()],
            program_id,
        );

        let offer_info = Offer {
            bump,
            maker: *maker.key,
            id: args.id,
            token_b_wanted_amount: args.token_b_wanted_amount,
            token_mint_a: *token_mint_a.key,
            token_mint_b: *token_mint_b.key,
        };

        let size = offer_info.try_to_vec()?.len();

        // create the Offer Account
        let lamports_required = (Rent::get()?).minimum_balance(size);

        solana_program::msg!("lamports required: {}", lamports_required);
        solana_program::msg!("bump: {}", bump);

        solana_program::msg!("create offer acccount");

        // create account
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                offer.key,
                lamports_required,
                size as u64,
                program_id,
            ),
            &[payer.clone(), offer.clone(), system_program.clone()],
            &[&[
                Offer::SEED_PREFIX.as_bytes(),
                maker.key.as_ref(),
                args.id.to_le_bytes().as_ref(),
                &[bump],
            ]],
        )?;

        solana_program::msg!("create vault token acccount");

        // create the vault token account
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                offer.key,
                token_mint_a.key,
                token_program.key,
            ),
            &[
                token_mint_a.clone(),
                vault.clone(),
                offer.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;

        solana_program::msg!("Tranfer to vault");

        // transfer mint a tokens to vault
        invoke(
            &token_instruction::transfer(
                token_program.key,
                maker_token_account_a.key,
                vault.key,
                maker.key,
                &[maker.key],
                args.token_a_offered_amount,
            )?,
            &[
                token_program.clone(),
                maker_token_account_a.clone(),
                vault.clone(),
                maker.clone(),
            ],
        )?;

        solana_program::msg!("CPI done. Writign to account");

        let token_amount_a = TokenAccount::unpack(&vault.data.borrow())?.amount;
        solana_program::msg!("Aount in vault: {}", token_amount_a);

        let offer_info = Offer {
            bump,
            maker: *maker.key,
            id: args.id,
            token_b_wanted_amount: args.token_b_wanted_amount,
            token_mint_a: *token_mint_a.key,
            token_mint_b: *token_mint_b.key,
        };

        offer_info.serialize(&mut *offer.data.borrow_mut())?;

        solana_program::msg!("Aount in vault: {:?}", offer.lamports());

        Ok(())
    }
}
