import { Account, Pubkey, Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class CreateAccountProgram {
  static PROGRAM_ID = new Pubkey('2QW7eymLrxC1TJmjTqtfnwvD8ND4dmVuHziX5p6sWzjj');

  //Create a new system account
  createSystemAccount(account: AccountState, owner: Signer): Result {
    //We use derive to define the account and chain the `.init()` at the end for creating the account
    account.derive(['account']).init(owner);
    //Set owner of the account
    account.owner = owner.key;

    account.accountBump = account.getBump();
  }
}

export interface AccountState extends Account {
  owner: Pubkey; // Owner of the account
  accountBump: u8; // Bump for the derived account
}
