use super::SteelInstruction;
use crate::borsh_instruction;
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{instructions as mpl_instruction, types::DataV2};
use solana_program::{msg, program::invoke, program_pack::Pack, rent::Rent, system_instruction};
use spl_token::state::Mint;
use steel::*;

// Using borsh for easier handling of dynamic types e.g strings
borsh_instruction!(SteelInstruction, CreateToken);
/// Create Token Instruction
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateToken {
    token_name: String,
    token_symbol: String,
    token_uri: String,
    decimals: u8,
}

impl CreateToken {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = Self::try_from_slice(data)?;

        let [mint_account, mint_authority, metadata_account, payer, rent, system_program, token_program, _token_metadata_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // First create the account for the Mint
        //
        msg!("Creating mint account...");
        msg!("Mint: {}", mint_account.key);
        invoke(
            &system_instruction::create_account(
                payer.key,
                mint_account.key,
                (Rent::get()?).minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
            ],
        )?;

        // Now initialize that account as a Mint (standard Mint)
        //
        msg!("Initializing mint account...");
        msg!("Mint: {}", mint_account.key);

        initialize_mint(
            mint_account,
            mint_authority,
            Some(mint_authority),
            token_program,
            rent,
            args.decimals,
        )?;

        // Now create the account for that Mint's metadata
        //
        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", metadata_account.key);

        let ix = &mpl_instruction::CreateMetadataAccountV3 {
            metadata: *metadata_account.key,
            mint: *mint_account.key,
            mint_authority: *mint_authority.key,
            payer: *payer.key,
            rent: None,
            system_program: *system_program.key,
            update_authority: (*payer.key, true),
        }
        .instruction(mpl_instruction::CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: args.token_name,
                symbol: args.token_symbol,
                uri: args.token_uri,
                creators: None,
                seller_fee_basis_points: 0,
                collection: None,
                uses: None,
            },
            collection_details: None,
            is_mutable: false,
        });

        invoke(
            ix,
            &[
                metadata_account.clone(),
                mint_account.clone(),
                mint_authority.clone(),
                payer.clone(),
                payer.clone(),
                system_program.clone(),
                rent.clone(),
            ],
        )?;

        msg!("Token mint created successfully.");

        Ok(())
    }
}
