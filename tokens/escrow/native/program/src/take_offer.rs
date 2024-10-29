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
    },
    spl_associated_token_account::instruction as associated_token_account_instruction,
    spl_token::{instruction as token_instruction, state::Account as TokenAccount},
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TakeOffer {}

impl TakeOffer {
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [offer, token_mint_a, token_mint_b, maker_token_account_b, taker_token_account_a, taker_token_account_b, vault, maker, taker, payer, token_program, associated_token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        let offer_data = Offer::try_from_slice(&offer.data.borrow()[..])?;

        if taker_token_account_a.lamports() == 0 {
            // create the vault token account
            invoke(
                &associated_token_account_instruction::create_associated_token_account(
                    payer.key,
                    taker.key,
                    token_mint_a.key,
                    token_program.key,
                ),
                &[
                    token_mint_a.clone(),
                    taker_token_account_a.clone(),
                    taker.clone(),
                    payer.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    associated_token_program.clone(),
                ],
            )?;
        }

        if maker_token_account_b.lamports() == 0 {
            // create the vault token account
            invoke(
                &associated_token_account_instruction::create_associated_token_account(
                    payer.key,
                    maker.key,
                    token_mint_b.key,
                    token_program.key,
                ),
                &[
                    token_mint_b.clone(),
                    maker_token_account_b.clone(),
                    maker.clone(),
                    payer.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    associated_token_program.clone(),
                ],
            )?;
        }

        let vault_amount_a = TokenAccount::unpack(&vault.data.borrow())?.amount;
        let taker_amount_a = TokenAccount::unpack(&taker_token_account_a.data.borrow())?.amount;
        let maker_amount_b = TokenAccount::unpack(&maker_token_account_b.data.borrow())?.amount;
        let taker_amount_b = TokenAccount::unpack(&taker_token_account_b.data.borrow())?.amount;

        solana_program::msg!("Vault A Balance Before Transfer: {}", vault_amount_a);
        solana_program::msg!("Taker A Balance Before Transfer: {}", taker_amount_a);
        solana_program::msg!("Maker B Balance Before Transfer: {}", maker_amount_b);
        solana_program::msg!("Taker B Balance Before Transfer: {}", taker_amount_b);

        // transfer mint a tokens to vault
        invoke(
            &token_instruction::transfer(
                token_program.key,
                taker_token_account_b.key,
                maker_token_account_b.key,
                taker.key,
                &[taker.key],
                offer_data.token_b_wanted_amount,
            )?,
            &[
                token_program.clone(),
                taker_token_account_b.clone(),
                maker_token_account_b.clone(),
                taker.clone(),
            ],
        )?;

        // transfer from vault to taker
        invoke_signed(
            &token_instruction::transfer(
                token_program.key,
                vault.key,
                taker_token_account_a.key,
                offer.key,
                &[offer.key, taker.key],
                vault_amount_a,
            )?,
            &[
                token_mint_a.clone(),
                vault.clone(),
                taker_token_account_a.clone(),
                offer.clone(),
                taker.clone(),
                token_program.clone(),
            ],
            &[&[
                b"offer",
                maker.key.as_ref(),
                offer_data.id.to_be_bytes().as_ref(),
                &[offer_data.bump],
            ]],
        )?;

        let vault_amount_a = TokenAccount::unpack(&vault.data.borrow())?.amount;
        let taker_amount_a = TokenAccount::unpack(&taker_token_account_a.data.borrow())?.amount;
        let maker_amount_b = TokenAccount::unpack(&maker_token_account_b.data.borrow())?.amount;
        let taker_amount_b = TokenAccount::unpack(&taker_token_account_b.data.borrow())?.amount;

        solana_program::msg!("Vault A Balance After Transfer: {}", vault_amount_a);
        solana_program::msg!("Taker A Balance After Transfer: {}", taker_amount_a);
        solana_program::msg!("Maker B Balance After Transfer: {}", maker_amount_b);
        solana_program::msg!("Taker B Balance After Transfer: {}", taker_amount_b);

        invoke_signed(
            &spl_token::instruction::close_account(
                token_program.key,
                vault.key,
                taker.key,
                offer.key,
                &[],
            )?,
            &[vault.clone(), taker.clone(), offer.clone()],
            &[&[
                b"offer",
                maker.key.as_ref(),
                offer_data.id.to_be_bytes().as_ref(),
                &[offer_data.bump],
            ]],
        )?;

        let token_amount_a = TokenAccount::unpack(&maker_token_account_b.data.borrow())?.amount;

        solana_program::msg!("log amount: {}", token_amount_a);
        solana_program::msg!("close prohram account");

        let lamports = offer.lamports();
        // Send the rent back to the payer
        **offer.lamports.borrow_mut() -= lamports;
        **payer.lamports.borrow_mut() += lamports;

        // Realloc the account to zero
        offer.realloc(0, true)?;

        // Assign the account to the System Program
        offer.assign(system_program.key);

        Ok(())
    }
}
