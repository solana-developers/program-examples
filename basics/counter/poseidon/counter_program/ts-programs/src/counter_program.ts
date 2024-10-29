import { Account, Pubkey, Signer, u8, u64 } from '@solanaturbine/poseidon';

export default class CounterProgram {
  static PROGRAM_ID = new Pubkey('3DRpGvotDMHtXzHahF1jdzYEiYa52cwpQcqGiNPw9vRd');

  initializeCounter(payer: Signer, counter: Counter) {
    counter.derive(['count', payer.key]).init();
    counter.count = new u64(0);

    counter.payer = payer.key;
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
