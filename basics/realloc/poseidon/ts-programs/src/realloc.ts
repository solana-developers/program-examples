import { Account, Pubkey, Result, Signer, SystemAccount, u8 } from '@solanaturbine/poseidon';

// Realloc Program:
// This program demonstrates how to handle dynamic fields stored in PDAs (Program Derived Addresses).
// However, due to current limitations of Poseidon, dynamic allocation isn't possible.
// Below is an example of a realloc program with fixed-sized fields.

export default class ReallocExample {
  static PROGRAM_ID = new Pubkey('2TVLNyk3jZCVNQ5UVJQRFPdjY3APCToU77isidjB3re4');

  initialize(payer: Signer, account: MessageAccountState, input: string): Result {
    account.derive(['message']).init();

    account.message = input;

    account.bump = account.getBump();
  }

  update(payer: Signer, account: MessageAccountState, input: string): Result {
    account.derive(['message']);

    account.message = input;
  }
}

export interface MessageAccountState extends Account {
  message: string;

  bump: u8;
}
