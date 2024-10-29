import { Account, String as PoseidonString, Pubkey, Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey('DDhy3V9AQE4wrJ3DXC5Yop2p56J6TiBRaz4zjnYnK8ao');

  createUser(user: Signer, userState: UserState, name: PoseidonString<50>): Result {
    userState.derive(['USER', user.key]).init();

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
  name: PoseidonString<50>;
  user: Pubkey;
}
