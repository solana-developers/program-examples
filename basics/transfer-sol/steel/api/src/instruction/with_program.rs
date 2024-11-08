use crate::TransferInstruction;
use steel::*;

instruction!(TransferInstruction, TransferSolWithProgram);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithProgram {
    amount: u64,
}

impl TransferSolWithProgram {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [payer, recipient] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.has_owner(&crate::ID)?;
        recipient.is_writable()?;

        // trasfer lamports to rent vault
        payer.send(args.amount, recipient);

        Ok(())
    }
}
