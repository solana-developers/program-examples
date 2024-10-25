import {
  Account,
  Signer,
  Pubkey,
  u8,
  type Result,
} from "@solanaturbine/poseidon";

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey("JxT1KDtKktz8Hv6GMjMkL2FNu7BmrD17brHtdERAunH");

  // Creating `CreateUserAccount` instruction
  // Parameters: All the accounts needed by the instruction
  createUserAccount(user: Signer, userAccount: AccountState): Result {
    // Use `.derive([seed])` to define the PDA and chain the `.init()` at the end for creating the account
    userAccount.derive(["user", user.key]).init();

    // Assign the values of the userAccount fields
    userAccount.bump = userAccount.getBump();
    userAccount.user = user.key;
  }

  // Creating `CloseUserAccount` instruction
  // Parameters: All the accounts needed by the instruction
  closeUserAccount(user: Signer, userAccount: AccountState): Result {
    // Chain `.close(<destination>)` after `.derive()` for closing accounts
    userAccount.derive(["user", user.key]).close(user);
  }
}

// Creating the `AccountState` state for the program
export interface AccountState extends Account {
  user: Pubkey;
  bump: u8;
}
