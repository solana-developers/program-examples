use steel::*;

use crate::state::User;

/// Instuction Discriminators
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    CreateUser = 0,
    CloseUser = 1,
}

instruction!(SteelInstruction, CreateUser);
/// Create User Instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateUser {
    name: [u8; 48],
}

impl CreateUser {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'_>],
        data: &[u8],
    ) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [user_info, payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        user_info.is_writable()?.has_seeds(
            &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
            program_id,
        )?;

        // create the user account
        create_account::<User>(
            user_info,
            system_program,
            payer,
            program_id,
            &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
        )?;

        let user = user_info.as_account_mut::<User>(program_id)?;

        *user = User { name: args.name };

        Ok(())
    }
}

instruction!(SteelInstruction, CloseUser);
/// Close User Instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CloseUser {}

impl CloseUser {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [user_info, payer, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // ensure it's the same account
        user_info.is_writable()?.has_seeds(
            &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
            program_id,
        )?;

        // close the program account, transfer lamports to payer
        user_info.close(payer)
    }
}
