import { Account, Pubkey, Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class CreateSystemAccountProgram {
  static PROGRAM_ID = new Pubkey('2Gs21s6ovwaHddKdPZvGpowpVJJBohdy3DrjoX77rqiY');

  //Create a new system account
  createSystemAccount(account: AccountState, owner: Signer): Result {
    //We use derive to define the account and chain the `.init()` at the end for creating the account
    account.derive(['account']).init();
    //Set owner of the account
    account.owner = owner.key;

    // Store bump for the account
    account.accountBump = account.getBump();
  }
}

export interface AccountState extends Account {
  owner: Pubkey; // Owner of the account
  accountBump: u8; // Bump for the derived account
}
