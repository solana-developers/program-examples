use crate::SteelInstruction;
use steel::*;

instruction!(SteelInstruction, TransferSolWithCpi);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSolWithCpi {
    amount: u64,
}

impl TransferSolWithCpi {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [payer, recipient, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_signer()?;
        recipient.is_writable()?;
        system_program.is_program(&system_program::ID)?;

        // trasfer lamports to rent vault
        recipient.collect(args.amount, payer)?;

        Ok(())
    }
}
