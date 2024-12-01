use {
    crate::error::*,
    mpl_token_metadata::{instructions as mpl_instruction, types::DataV2},
    solana_program::{msg, program::invoke, program_pack::Pack, rent::Rent, system_instruction},
    spl_token::state::Mint,
    std::ffi::CStr,
    steel::*,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CreateToken = 0,
}

instruction!(SteelInstruction, CreateToken);
// CreateToken instruction
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateToken {
    pub token_name: [u8; 32],   // Metaplex metadata name: 32 bytes max
    pub token_symbol: [u8; 10], // Metaplex metadata symbol: 10 bytes max
    pub token_uri: [u8; 256],   // Metaplex metadata uri: 200 bytes max
    pub decimals: u8,
}

impl CreateToken {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [mint_account, mint_authority, metadata_account, payer, rent, system_program, token_program, token_metadata_program] =
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

        let name = Self::str_from_bytes(&mut args.token_name.to_vec())?.to_string();
        let symbol = Self::str_from_bytes(&mut args.token_symbol.to_vec())?.to_string();
        let uri = Self::str_from_bytes(&mut args.token_uri.to_vec())?.to_string();

        mpl_instruction::CreateMetadataAccountV3Cpi {
            __program: token_metadata_program,
            metadata: metadata_account,
            mint: mint_account,
            mint_authority,
            payer,
            update_authority: (mint_authority, true),
            system_program,
            rent: Some(rent),
            __args: mpl_instruction::CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        }
        .invoke()?;

        msg!("Token mint created successfully.");

        Ok(())
    }

    fn str_from_bytes(bytes: &mut Vec<u8>) -> Result<&str, ProgramError> {
        // add an extra null byte, in the case every position is occupied with a non-null byte
        bytes.push(0);

        // remove excess null bytes
        if let Ok(cstr) = CStr::from_bytes_until_nul(bytes) {
            if let Ok(str) = cstr.to_str() {
                return Ok(str);
            }
        }
        Err(SteelError::ParseError.into())
    }
}
