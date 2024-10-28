import {
  Account,
  String as PString, // Had to create an alias otherwise, staged lint:check
  Pubkey,
  type Result,
  Signer,
  u32,
} from '@solanaturbine/poseidon';

/**
 * ProcessingInstructions class handles the instructions for the Poseidon program.
 * It includes methods to interact with user accounts and process specific instructions.
 *
 * Since currently, Poseidon doesn't implement rust macros (e.g. msg!())
 * we'll be doing a different but same approach from the original anchor framework example
 */
export default class ProcessingInstructions {
  static PROGRAM_ID = new Pubkey('AthFDPn5w6LJLhez7ya4dcokjjZGLPozCPn3RUJrCkZ8');

  // goToPark method processes the GoToPark instruction on the program.
  goToPark(
    // ACCOUNTS & INSTRUCTION DATA
    payer: Signer,
    user: UserAccount,

    // The same with original anchor framework, just put instruction data in the function parameters
    // that's how instructions are processed also here in Poseidon
    name: PString<25>,
    height: u32,
  ): Result {
    // CONTEXT

    // .derive() ensures that the user account will be a PDA derived from the parameters as seeds
    // .init() ensures that the init constraint will properly included for the account initialization
    user.derive([payer.key]).init();

    // Assign the provided name and height to the user account
    user.name = name;
    user.height = height;
  }
}

// STATE ACCOUNTS
export interface UserAccount extends Account {
  name: PString<25>; // The name of the user, limited to 25 characters.
  height: u32;
}
