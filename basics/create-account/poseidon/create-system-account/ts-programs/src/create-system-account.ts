import { Account, Pubkey, Result, Signer, SystemAccount, SystemProgram, UncheckedAccount, u8 } from '@solanaturbine/poseidon';

export default class CreateSystemAccountProgram {
  static PROGRAM_ID = new Pubkey('HiodPTcV4ZBV8GkqNPRhJKuVoBAxzEQYxK2Mbv9i9vY4');

  // Initialize a new system account
  initialize(state: AccountState, owner: Signer, auth: UncheckedAccount): Result {
    // Create a new account with derived state
    state.derive(['account']).init();

    // Set the owner of the account
    state.owner = owner.key;

    // Store bumps for the account
    state.authBump = auth.getBump();
    state.accountBump = state.getBump();
  }

  // Update account state
  update(state: AccountState, newValue: u8): Result {
    state.derive(['account']); // Ensure we're working with the correct derived account
    state.value = newValue; // Update the account state with the new value
  }

  delete(state: AccountState, signer: Signer): Result {
    // Derive the correct account and check ownership
    state.derive(['account']).has([signer.key]).close(signer);
  }
}

// Define the custom account state interface
export interface AccountState extends Account {
  owner: Pubkey; // Owner of the account
  value: u8; // Value to store in the account
  accountBump: u8; // Bump for the derived account
  authBump: u8; // Bump for the authentication
}
