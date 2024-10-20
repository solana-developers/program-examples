import { VOTE_PROGRAM_ID } from '@solana/web3.js';
import { Account, Pubkey, Result, Signer, u8, UncheckedAccount } from '@solanaturbine/poseidon';

 export default class CheckAccountsProgram {
   static PROGRAM_ID = new Pubkey(
     "DqZo8ioCBtRiFibxQeWrHUtE8ZES5ETA6Uq3hgAYWsUD"
   );

   // Account validation in poseidon is done using the types and constraints specified in the method parameters
   // This is a simple example and does not include all possible constraints and types
   checkAccounts(
     owner:Pubkey,
     payer:Signer,
     account_to_create: UncheckedAccount,
     account_to_change: UncheckedAccount
   ): Result {
     account_to_create.derive(["account"]).init();
     account_to_change.derive(["change"]).has([CheckAccountsProgram.PROGRAM_ID])
 }
 }
export interface AccountState extends Account {
    owner:Pubkey
}
