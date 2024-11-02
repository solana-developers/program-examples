use steel::*;

declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, _data) = parse_instruction(&crate::ID, program_id, data)?;

    match ix {
        CounterInstruction::Initialize => Initialize::process(accounts),
        CounterInstruction::Increment => Increment::process(accounts),
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CounterInstruction {
    Initialize = 0,
    Increment = 1,
}

instruction!(CounterInstruction, Initialize);
// Initialize
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

impl Initialize {
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [counter_account, payer, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        payer.is_writable()?.is_signer()?; // make use payer is writable and signer
        system_program.is_program(&system_program::ID)?; // system program check
        counter_account
            .is_writable()? // check the account is writable
            .has_seeds(&[b"counter"], &crate::ID)?; // check the address is derived from the right seeds

        // create the counter account
        create_account::<Counter>(
            counter_account,  // account to be created
            system_program,   // system program
            payer,            // payer
            &crate::ID,       // program id
            &[COUNTER_SEEDS], // seeds
        )?;

        let counter = counter_account.as_account::<Counter>(&crate::ID)?;
        let count = counter.count;

        solana_program::msg!("Counter initialized! Count is {}", count);

        Ok(())
    }
}

instruction!(CounterInstruction, Increment);
// Increment
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Increment {}

impl Increment {
    pub fn process(accounts: &[AccountInfo<'_>]) -> ProgramResult {
        let [counter_account] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        counter_account
            .is_writable()? // check the account is writable
            .has_seeds(&[COUNTER_SEEDS], &crate::ID)?; // check the address is derived from the right seeds

        let counter = counter_account.as_account_mut::<Counter>(&crate::ID)?;
        counter.count += 1;

        let count = counter.count;

        solana_program::msg!("Counter state incremented to {:?}", count);

        Ok(())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum CounterAccount {
    Counter = 0,
}

account!(CounterAccount, Counter);
// Counter
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub count: u64,
}

pub const COUNTER_SEEDS: &[u8] = b"counter"; // counter seeds
