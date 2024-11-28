use borsh::BorshDeserialize;
use pda_rent_payer_api::prelude::*;
use solana_program::msg;
use steel::*;

pub fn process_initialize_vault(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args
    let args = InitializeRentVault::try_from_slice(data)?;

    // Load and validate accounts.
    let [payer_info, rent_vault_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    payer_info.is_signer()?;
    rent_vault_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[RENT_VAULT], &pda_rent_payer_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize vault.
    create_account::<RentVault>(
        rent_vault_info,
        system_program,
        payer_info,
        &pda_rent_payer_api::ID,
        &[RENT_VAULT],
    )?;

    let (_, bump) = rent_vault_pda();

    // Get account to see if it's created
    let _vault = rent_vault_info.as_account_mut::<RentVault>(&pda_rent_payer_api::ID)?;

    let transfer = solana_program::program::invoke_signed(
        &solana_program::system_instruction::transfer(
            payer_info.key,
            rent_vault_info.key,
            args.amount,
        ),
        &[
            payer_info.clone(),
            rent_vault_info.clone(),
            system_program.clone(),
        ],
        &[&[RENT_VAULT, &[bump]]],
    );

    let vault_balance = rent_vault_info.lamports();
    msg!("Updated vault balance: {}", vault_balance);

    match transfer {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    msg!("Initialized rent vault.");
    msg!("PDA: {:?}", rent_vault_info.key);

    Ok(())
}
