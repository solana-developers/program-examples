import { Account, Pubkey, Result, Signer, u8, u32 } from '@solanaturbine/poseidon';

export default class ProgramDerivedAddresses {
  static PROGRAM_ID = new Pubkey('F1TXTbegcoBMBNFJPp8QVx9eMnh62qDSNMWU2FVWqV5i');

  createPageVisits(pageVisits: PageVisit, payer: Signer): Result {
    pageVisits.derive(['page_visits', payer.key]).init(payer);
    pageVisits.bump = pageVisits.getBump();
    pageVisits.pageVisits = new u32(0);
  }

  increment(pageVisits: PageVisit, user: Signer): Result {
    pageVisits.derive(['page_visits', user.key]);
    pageVisits.pageVisits = pageVisits.pageVisits.add(1);
  }
}

export interface PageVisit extends Account {
  pageVisits: u32;
  bump: u8;
}
