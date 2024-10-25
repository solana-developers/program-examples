import {
  Account,
  Pubkey,
  type Result,
  Signer,
  u64,
} from "@solanaturbine/poseidon";

export default class Counter {
  static PROGRAM_ID = new Pubkey("EqkjKELHRgipbqzTL4x6uo9hWWoMuZ7G8vdWh6A2cpj");

  initializeCounter(payer: Signer, counter: CounterState): Result {
    counter.derive([""]).init();
    counter.count = new u64(0);
  }

  incrementCounter(counter: CounterState): Result {
    counter.derive([""]);
    counter.count = counter.count.add(1);
  }
}

export interface CounterState extends Account {
  count: u64;
}
