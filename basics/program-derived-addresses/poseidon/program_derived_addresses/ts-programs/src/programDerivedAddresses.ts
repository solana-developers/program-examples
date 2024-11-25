import { Account, Pubkey, Result, Signer, u8, u32 } from '@solanaturbine/poseidon';

export default class ProgramDerivedAddresses {
  static PROGRAM_ID = new Pubkey('BW3PD9JUZzMbSEGmZZBRncwMqPgyUpsY1yYbWMDV7aRf');

  createPageVisit(state: PageVisit, payer: Signer): Result {
    state.derive(['page_visits', payer.key]).init();
    state.bump = state.getBump();
    state.pageVisits = new u32(0);
  }

  increment(state: PageVisit, user: Signer): Result {
    state.derive(['page_visits', user.key]);
    state.pageVisits = state.pageVisits.add(1);
  }
}

export interface PageVisit extends Account {
  pageVisits: u32;
  bump: u8;
}
