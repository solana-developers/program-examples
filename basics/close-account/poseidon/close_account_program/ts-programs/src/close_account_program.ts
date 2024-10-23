import { Account, Pubkey, Result, Signer, u8, u32 } from '@solanaturbine/poseidon';

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey('2q4uoWExFAbZjeDe4n3miipsT9bX9vLnkSetCfZYF2VT');

  createUser(user: Signer, userAccount: UserAccount, userId: u32): Result {
    userAccount.derive(['USER', user.key]).init();

    userAccount.userBump = userAccount.getBump();

    userAccount.user = user.key;

    userAccount.userId = userId;
  }
  closeUser(userAccount: UserAccount, user: Signer): Result {
    userAccount.derive(['USER', user.key]).close(user);
  }
}

export interface UserAccount extends Account {
  userBump: u8;
  userId: u32;
  user: Pubkey;
}
