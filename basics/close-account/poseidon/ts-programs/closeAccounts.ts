import { Account, Pubkey, Result, u8, Signer } from "@solanaturbine/poseidon";

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey("11111111111111111111111111111111");

  initalize(state: AccountState, user: Signer, data: u8): Result {
    state.derive(["account"]).init();
    state.someData = data;
  }
  close(state: AccountState, user: Signer): Result {
    state.derive(["account"]).close(user);
  }
}

export interface AccountState extends Account {
  someData: u8;
}
