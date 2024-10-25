import { Account, Pubkey, Result, Signer, u8 } from "@solanaturbine/poseidon";

export default class CloseAccount {
  static PROGRAM_ID = new Pubkey(
    "3bo4FzxB1xAiPqZjq8nmLvHGMEsqm96VZeJFXFei7DJD"
  );

  // create user account
  createUser(userAccount: CloseAccountState, user: Signer): Result {
    userAccount.derive(["user", user.key]).init();

    // set the initial values
    userAccount.user = user.key;
    userAccount.bump = userAccount.getBump();
  }

  // close user account
  closeUser(userAccount: CloseAccountState, user: Signer): Result {
    userAccount.derive(["user", user.key]).close(user);
  }
}

export interface CloseAccountState extends Account {
  user: Pubkey; // This field store the user pub key
  bump: u8; // bump is for PDA (program derieved account, a special type of account which controlled by program on Solana)
}
