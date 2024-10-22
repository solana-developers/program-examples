import { Account, Mint, Pubkey, Result, Signer, SystemAccount, SystemProgram, TokenAccount, u8, u64 } from '@solanaturbine/poseidon';

export default class PdaRentPayer {
  static PROGRAM_ID = new Pubkey('BYj8GpV9hpv9PAVdwoWFCTMkysJkk5jstYjuCrw4pxem');

  // When lamports are transferred to a new address (without and existing account),
  // An account owned by the system program is created by default
  initRentVault(owner: Signer, rent_vault: RentState, fund_lamports: u64) {
    //Derive and intialize the vault account with bump and seeds
    rent_vault.derive(['rent_vault', owner.key]).init();

    rent_vault.rentVaultBump = rent_vault.getBump();
    // Transfer specified lamports from payer to the rent vault to initialize the rent vault
    SystemProgram.transfer(owner, rent_vault, fund_lamports, ['rent_vault', owner.key, rent_vault.rentVaultBump]);
  }

  // Create a new system account
  createNewAccount(owner: Signer, rent_vault: RentState, new_account: AccountState, transfer_amount: u64): Result {
    //We use derive to define the account and chain the `.init()` at the end for creating the account
    new_account.derive(['account', owner.key]).init();

    //Set owner of the account
    new_account.owner = owner.key;

    // Store bump for the account
    new_account.accountBump = new_account.getBump();

    // We now transfer the lamports from the rent_vault to the new account
    SystemProgram.transfer(rent_vault, new_account, transfer_amount, ['rent_vault', owner.key, rent_vault.rentVaultBump]);
  }
}

export interface AccountState extends Account {
  owner: Pubkey; // Owner of the account
  accountBump: u8; // Bump for the derived account
}

export interface RentState extends Account {
  rentVaultBump: u8; // Bump for rent Vault Pda
}
