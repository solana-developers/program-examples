import { Account, Pubkey, Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey('AXXZKfVRcFtAkRbk3oZU8rVStmP2nRY54xky1C7Mi6mb');

  // create user account
  createUser(userAccount: CloseAccountState, user: Signer): Result {
    userAccount.derive(['user', user.key]).init(user);

    // set the initial values
    userAccount.user = user.key;
    userAccount.bump = userAccount.getBump();
  }

  // close user account
  closeUser(userAccount: CloseAccountState, user: Signer): Result {
    userAccount.derive(['user', user.key]).close(user);
  }
}

export interface CloseAccountState extends Account {
  user: Pubkey;
  bump: u8;
}
