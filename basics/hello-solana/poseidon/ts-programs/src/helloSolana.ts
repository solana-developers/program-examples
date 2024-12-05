import { Account, Pubkey, Signer, u8 } from '@solanaturbine/poseidon';

export default class HelloSolana {
  static PROGRAM_ID = new Pubkey('AAfRjjKbh77KLxouxEymo5uzJ4qbRaqorx5gHVuX59o8');

  initialize(authority: Signer, counter: Counter) {
    // Initialize the counter
    counter.derive(['counter', authority.key]).init(authority);

    // Set the counter authority
    counter.authority = authority.key;

    // Note: Poseidon does not support the msg!() macro at the point of writing, so this gets ignored when compiled to Anchor.
    console.log('Hello, Solana from Poseidon!');
    console.log(`Counter initialized for ${counter.authority}`);
  }

  increment(authority: Signer, counter: Counter) {
    counter.value = counter.value.add(1);
  }
}

export interface Counter extends Account {
  authority: Pubkey;
  value: u8;
}
