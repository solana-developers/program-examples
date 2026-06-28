use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use pinocchio_log::log;

use crate::instructions::{create_collection, mint_nft, verify_collection};

/// Dispatches an instruction based on its leading discriminator byte.
///
/// Instruction data layout: `[discriminator: u8, authority_bump: u8]`
///   - `0` -> CreateCollection  (mints a collection NFT)
///   - `1` -> MintNft           (mints an NFT that belongs to the collection)
///   - `2` -> VerifyCollection  (verifies the NFT as part of the collection)
///
/// Every instruction carries the bump of the `[b"authority"]` mint-authority
/// PDA so the program can sign the Metaplex CPIs with it (the Anchor example
/// derives this bump on-chain instead).
pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, args) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match *discriminator {
        0 => {
            log!("Instruction: CreateCollection");
            create_collection(program_id, accounts, args)
        }
        1 => {
            log!("Instruction: MintNft");
            mint_nft(program_id, accounts, args)
        }
        2 => {
            log!("Instruction: VerifyCollection");
            verify_collection(program_id, accounts, args)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
