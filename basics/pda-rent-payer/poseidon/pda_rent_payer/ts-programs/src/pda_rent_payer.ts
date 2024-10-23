import { Account, Mint, Pubkey, Result, Signer, SystemAccount, SystemProgram, UncheckedAccount, u8, u64 } from '@solanaturbine/poseidon';

export default class PdaRentPayer {
  static PROGRAM_ID = new Pubkey('BYj8GpV9hpv9PAVdwoWFCTMkysJkk5jstYjuCrw4pxem');

  // When lamports are transferred to a new address (without and existing account),
  // An account owned by the system program is created by default
  initRentVault(owner: Signer, vault: RentVault, state: RentAccountState, auth: UncheckedAccount) {
    //Derive the accounts with bump and seeds
    vault.derive(['vault', auth.key]).init();
    state.derive(['state', owner.key]).init();
    auth.derive(['auth', state.key]);

    state.owner = owner.key;

    //To store bumps in the RentAccountState, we simply call getBump on the RentAccountState
    state.stateBump = state.getBump();
    state.authBump = auth.getBump();
    state.vaultBump = vault.getBump();
  }

  depositToRentVault(owner: Signer, state: RentAccountState, auth: UncheckedAccount, vault: RentVault, amount: u64) {
    //Since we have stored bumps in the RentAccountState we can derive PDAs with stored bumps by passing that as the second argument
    state.deriveWithBump(['state', owner.key], state.stateBump);
    auth.deriveWithBump(['auth', state.key], state.authBump);
    vault.deriveWithBump(['vault', auth.key], state.vaultBump);

    // Transfer specified lamports from owner to the rent vault
    SystemProgram.transfer(
      owner, //from
      vault, //to
      amount, //amount to be sent
    );
  }

  //Deposit some sol into out PDA to pay for rent
  createNewAccount(
    owner: Signer,
    state: RentAccountState,
    auth: UncheckedAccount,
    vault: SystemAccount,
    new_account_state: NewAccountState,
    amount: u64,
  ): Result {
    new_account_state.derive(['new_account', owner.key]).init();

    state.deriveWithBump(['state', owner.key], state.stateBump);
    auth.deriveWithBump(['auth', state.key], state.authBump);
    vault.deriveWithBump(['vault', auth.key], state.vaultBump);

    // state.newAccountBump = new_account.getBump(); // we don't need the new_account bump

    new_account_state.owner = owner.key;

    // We now transfer the lamports from the rent_vault to the new account
    SystemProgram.transfer(vault, new_account_state, amount, ['vault', auth.key]);
  }
}

export interface RentAccountState extends Account {
  owner: Pubkey; // Owner of the account
  stateBump: u8; // Bump for the state account
  authBump: u8; // Bump for the auth account
  vaultBump: u8; // Bump for the vault account
}

//Our vault here is CustomStateAccount not a SystemAccount
export interface RentVault extends Account {}

export interface NewAccountState extends Account {
  owner: Pubkey;
  newAccountBump: u8; // Bump for the newly created account
}
