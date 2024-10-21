import { Account, Constraint, Pubkey, Result, Signer, UncheckedAccount, u8 } from '@solanaturbine/poseidon';

export default class CheckAccountsProgram {
  static PROGRAM_ID = new Pubkey('DqZo8ioCBtRiFibxQeWrHUtE8ZES5ETA6Uq3hgAYWsUD');

  // Account validation in poseidon is done using the types and constraints specified in the method parameters,
  // this is a simple example and does not include all possible constraints and types

  // The order and number of accounts are automatically checked based on the struct definition in the generated anchor code

  //Checking if the program ID from the instruction is the program ID of your program in Anchor is done automatically, as Anchor checks that accounts passed into the program are owned by the expected program.

  checkAccounts(payer: Signer, account_to_create: UncheckedAccount, account_to_change: UncheckedAccount): Result {}
}
