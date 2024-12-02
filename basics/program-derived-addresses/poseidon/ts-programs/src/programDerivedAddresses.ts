import { Account, Pubkey, type Result, Signer, u32, u64 } from '@solanaturbine/poseidon';

export default class ProgramDerivedAddresses {
  static PROGRAM_ID = new Pubkey('GBQw9SP64U2WYhRwwWCQswd4KPcK19cSSw7BvdxK9hyG');

  createPageVisits(
    // ACCOUNTS, CONTEXT & INSTRUCTION INPUTS

    payer: Signer,
    page_visits: PageVisits,
    seed: u64,
  ): Result {
    // INSTRUCTION LOGIC

    // .derive() ensures that the page visits is a PDA derived from the parameters as the seed
    // .init() ensures that the page visit will have the init constraint for initialization
    page_visits.derive([seed.toBytes(), payer.key]).init();

    // Set the initial page visits value to 0 and save the seed used for deriving the account
    page_visits.page_visits = new u32(0);
    page_visits.seed = seed;
  }

  incrementPageVisits(
    // ACCOUNTS, CONTEXT & INSTRUCTION INPUTS

    payer: Signer,
    page_visits: PageVisits,
  ): Result {
    // INSTRUCTION LOGIC

    // Here we derived again the page visits so it can be used for other logic
    page_visits.derive([page_visits.seed.toBytes(), payer.key]);

    // Increment the page visits count by 1
    page_visits.page_visits = page_visits.page_visits.add(1);
  }
}

// ACCOUNT STATES
export interface PageVisits extends Account {
  seed: u64; // The seed used for deriving the account address.
  page_visits: u32;
}
