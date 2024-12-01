use crate::{state::*, SteelInstruction};
use steel::*;

instruction!(SteelInstruction, CreateAmm);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAmm {
    id: Pubkey,
    fee: u16,
}

impl CreateAmm {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        data: &[u8],
    ) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [amm_info, admin, payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_writable()?;
        system_program.is_program(&system_program::ID)?;

        // create amm
        create_account::<Amm>(
            amm_info,
            system_program,
            payer,
            program_id,
            &[args.id.as_ref()],
        )?;

        let amm = amm_info.as_account_mut::<Amm>(program_id)?;

        amm.admin = *admin.key;
        amm.fee = args.fee;
        amm.id = args.id;

        Ok(())
    }
}
