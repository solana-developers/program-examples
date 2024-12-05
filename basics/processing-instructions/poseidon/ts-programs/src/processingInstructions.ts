import { Account, Pubkey, Signer, u64 } from '@solanaturbine/poseidon';

/**
 * NOTE: At the time of writing, Poseidon is a new framework and has a number of
 * limitations.
 */

export default class ProcessingInstructions {
  static PROGRAM_ID = new Pubkey('BWj4tkT21WrGfyi1hcYKjAKrB3UbXm144hZ84RaMLV7C');

  // Instruction: Initializes a new GreetingAccount with a custom message.
  initialize(
    payer: Signer,
    greeting: GreetingAccount,
    time: u64,
    // message: String // At the time of writing, Poseidon does not support string handling.
  ) {
    // Initialize Greeting Account
    greeting.derive(['greeting']).init();

    greeting.lastUpdated = time; // Ideally, the Clock would be used instead of obtaining the time from the client.
    // greeting.message = message; // At the time of writing, Poseidon does not support string handling.

    // console.log(`${signer.key} initialized greeting account with message: ${greeting.message}`); // At the time of writing, Poseidon does not support the Anchor msg! macro.
  }

  // Instruction: Updates the greeting message in the GreetingAccount.
  updateGreeting(
    greeting: GreetingAccount,
    time: u64,
    // new_message: String // At the time of writing, Poseidon does not support string handling.
  ) {
    greeting.lastUpdated = time; // Ideally, the Clock would be used instead of obtaining the time from the client.
    // greeting.message = new_message; // At the time of writing, Poseidon does not support string handling.

    // console.log(`${signer.key} updated greeting account with message: ${greeting.message}`); // At the time of writing, Poseidon does not support the Anchor msg! macro.
  }
}

export interface GreetingAccount extends Account {
  lastUpdated: u64;

  // greeting: String; // At the time of writing, Poseidon does not support string handling.
}
