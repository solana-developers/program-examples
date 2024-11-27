import { Account, Pubkey, Signer, u8, u64 } from '@solanaturbine/poseidon';

export default class CounterProgram {
  static PROGRAM_ID = new Pubkey('EgcUM7mn2dsedh9vjY8ihfzuU9Vhhady8bSPcRssUriR');

  initializeCounter(payer: Signer, counter: Counter) {
    counter.derive(['count', payer.key]).init(payer);
    counter.count = new u64(0);
  }
  increment(counter: Counter) {
    counter.count = counter.count.add(1);
  }
  decrement(counter: Counter) {
    counter.count = counter.count.sub(1);
  }
}

export interface Counter extends Account {
  payer: Pubkey;
  count: u64;
  bump: u8;
}
