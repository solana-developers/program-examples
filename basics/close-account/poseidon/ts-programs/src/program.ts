import { Account, Pubkey, Result, Signer, u8, u64 } from '@solanaturbine/poseidon';

export default class CloseAccountProgram {
  // define the progam id as a static constant like bellow
  static PROGRAM_ID = new Pubkey('HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1');

  createUser(user: Signer, user_account: UserState, name: string): Result {
    user_account.derive(['USER', user.key]).init();
    user_account.user = user.key;
    user_account.bump = user_account.getBump();
    user_account.name = new String('Hello');
  }

  closeUser(user: Signer, user_account: UserState): Result {
    user_account.derive([]).close(user);
    user_account.deriveWithBump(['USER', user.key], user_account.bump);
  }
}

export interface UserState extends Account {
  bump: u8;
  user: Pubkey;
  name: string;
  name_size: u64;
}
