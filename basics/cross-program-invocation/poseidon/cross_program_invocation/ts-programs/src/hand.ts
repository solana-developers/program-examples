import { Account, Pubkey, Result, Signer } from '@solanaturbine/poseidon';

export default class Hand {
  static PROGRAM_ID = new Pubkey('Cd86dtBUzQKYTFtcB8zDxPRUPCtKPocyetWZSnq6PNxv');

  initialize(user: Signer, power: Signer) {}

  pullLever(
    user: Signer,
    power: PowerStatus,
    // name: String
  ): Result {
    power.derive(['hand']).init();
  }
  // switchPower(name: String) {}
}

export interface PowerStatus extends Account {
  // is_on: bool
}
