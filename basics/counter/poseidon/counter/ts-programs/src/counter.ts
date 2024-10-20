import { Account, Pubkey, Result, Signer, u64 } from '@solanaturbine/poseidon';

export default class Counter {
  static PROGRAM_ID = new Pubkey('Hn5fB7seeqBPGWWXQCnoA4bomJy9H8ktX9f2HiGtGWP1');

  initialize(state: CounterState, payer: Signer): Result {
    state.derive(['count']).init();
    state.count = new u64(0);
  }

  increment(state: CounterState): Result {
    state.derive(['count']);
    state.count = state.count.add(1);
  }

  decrement(state: CounterState): Result {
    state.derive(['count']);
    state.count = state.count.sub(1);
  }
}

export interface CounterState extends Account {
  count: u64;
}
