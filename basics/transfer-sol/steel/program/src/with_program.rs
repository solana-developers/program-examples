use crate::SteelInstruction;
use steel::*;

instruction!(SteelInstruction, TransferSolWithProgram);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithProgram {
    amount: u64,
}

impl TransferSolWithProgram {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        data: &[u8],
    ) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [payer, recipient] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.has_owner(program_id)?;
        recipient.is_writable()?;

        // trasfer lamports to rent vault
        payer.send(args.amount, recipient);

        Ok(())
    }
}
