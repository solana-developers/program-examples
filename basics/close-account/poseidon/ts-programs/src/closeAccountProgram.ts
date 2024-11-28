import { Account, Pubkey, Result, Signer, String, u8 } from '@solanaturbine/poseidon';

export default class CloseAccountProgram {
  static PROGRAM_ID = new Pubkey('Cp1rfMVrJoD9aNT8dGVoPAf2BrY6HBMXbsTPfd2heV6C');

  createUser(user: Signer, userState: UserState, name: String<50>): Result {
    userState.derive(['USER', user.key]).init(user);

    userState.userBump = userState.getBump();

    userState.user = user.key;

    userState.name = name;
  }
  closeUser(userAccount: UserState, user: Signer): Result {
    userAccount.derive(['USER', user.key]).close(user);
  }
}

export interface UserState extends Account {
  userBump: u8;
  name: String<50>;
  user: Pubkey;
}
