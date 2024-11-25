import { Account, Pubkey, Result, Signer, u8 } from '@solanaturbine/poseidon';

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey('4So9Jbx672BRL9RvfB8Sux2NMVX5QJRnhmdWyij3kkFg');

  initalize(state: AccountState, user: Signer, data: u8): Result {
    state.derive(['account']).init();
    state.someData = data;
  }
  close(state: AccountState, user: Signer): Result {
    state.derive(['account']).close(user);
  }
}

export interface AccountState extends Account {
  someData: u8;
}
