#![no_std]

use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    nostd_panic_handler,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
nostd_panic_handler!();

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((&CREATE_DISCRIMINATOR, data)) => process_user(program_id, accounts, data),
        Some((&CLOSE_DISCRIMINATOR, _)) => process_close(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub const CREATE_DISCRIMINATOR: u8 = 0;
pub const CLOSE_DISCRIMINATOR: u8 = 1;

pub struct User<'a> {
    pub name: &'a [u8],
}

impl<'a> User<'a> {
    pub const SEED_PREFIX: &'static str = "USER";
    pub const LEN: usize = 16;
}

fn process_user(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [target_account, payer, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent::get()?;

    let account_span = User::LEN;
    let lamports_required = rent.minimum_balance(account_span);

    let (_, bump) = pinocchio::pubkey::find_program_address(
        &[User::SEED_PREFIX.as_bytes(), payer.key()],
        program_id,
    );
    let bump_bytes = bump.to_le_bytes();

    let seeds = [
        Seed::from(User::SEED_PREFIX.as_bytes()),
        Seed::from(payer.key().as_ref()),
        Seed::from(&bump_bytes),
    ];
    let signers = [Signer::from(&seeds)];

    CreateAccount {
        from: payer,
        to: target_account,
        lamports: lamports_required,
        space: account_span as u64,
        owner: program_id,
    }
    .invoke_signed(&signers)?;

    let mut address_info_data = target_account.try_borrow_mut_data()?;
    address_info_data.copy_from_slice(instruction_data);

    Ok(())
}

fn process_close(accounts: &[AccountInfo]) -> ProgramResult {
    let [target_account, payer, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent::get()?;

    let account_span = 0usize;
    let lamports_required = rent.minimum_balance(account_span);

    let diff = target_account.lamports() - lamports_required;

    *target_account.try_borrow_mut_lamports()? -= diff;
    *payer.try_borrow_mut_lamports()? += diff;

    target_account.resize(account_span)?;

    unsafe {
        target_account.assign(system_program.key());
    }

    Ok(())
}
