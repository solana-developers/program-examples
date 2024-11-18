import { Account, Pubkey, Result, Signer, u8, u32 } from '@solanaturbine/poseidon';

export default class ProgramDerivedAddresses {
  static PROGRAM_ID = new Pubkey('HcZYDcJ4AF3LgKnDaF6jKyBY7Lf2zggPn4vdGvHBACiW');

  createPageVisits(pageVisits: PageVisits, user: Signer): Result {
    pageVisits.derive(['page_visits', user.key]).init();

    pageVisits.pageVisits = new u32(0);
    pageVisits.bump = pageVisits.getBump();
  }

  incrementPageVisits(pageVisits: PageVisits): Result {
    pageVisits.pageVisits = pageVisits.pageVisits.add(1);
  }
}

export interface PageVisits extends Account {
  pageVisits: u32;
  bump: u8;
}
