import { Account, Pubkey, Result, Signer, Str, SystemAccount, u8 } from '@solanaturbine/poseidon';

// Note:
// Realloc Program:

// Due to current limitations in Poseidon, dynamic allocation (reallocation) is not supported on Poseidon right now.
// As a result, this example uses fixed-sized fields to work around the limitation.
// In typical Solana programs using Anchor, dynamic reallocation allows accounts to resize based on the input data.

export default class ReallocProgram {
  static PROGRAM_ID = new Pubkey('7T1DgawXjJD6kGaC43ujSw2xXLhn7w28MGzyD7oV8Q1B');

  initialize(payer: Signer, account: MessageAccountState, input: Str<25>): Result {
    account.derive(['message']).init(payer);

    account.message = input;

    account.bump = account.getBump();
  }

  update(payer: Signer, account: MessageAccountState, input: Str<25>): Result {
    account.derive(['message']);

    account.message = input;
  }
}

export interface MessageAccountState extends Account {
  message: Str<25>;

  bump: u8;
}
