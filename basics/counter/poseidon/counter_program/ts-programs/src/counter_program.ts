import { Account, Pubkey, Signer, u8, u64 } from '@solanaturbine/poseidon';

export interface Counter extends Account {
  count: u64;
  bump: u8;
}

export default class counter_program {
  static PROGRAM_ID = new Pubkey('7yvcNv9BAHHZYPgDag1YFSLEbXiwBTmVmuE4eArSSEKH');

  initialize_counter(counter: Counter, payer: Signer) {
    counter.derive(['count']);
    counter.count = new u64(0);
  }
  increment(counter: Counter) {
    counter.derive(['count']);
    counter.count = counter.count.add(1);
  }
}
