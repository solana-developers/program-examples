import { Account, Boolean, Pubkey, Result, Signer, String } from '@solanaturbine/poseidon';

export default class Hand {
  static PROGRAM_ID = new Pubkey('Cd86dtBUzQKYTFtcB8zDxPRUPCtKPocyetWZSnq6PNxv');

  initialize(user: Signer, power: Signer) {}

  pullLever(user: Signer, power: PowerStatus, name: String<10>): Result {
    power.derive(['hand']).init(user);
  }
  switchPower(name: String<10>) {}
}

export interface PowerStatus extends Account {
  is_on: Boolean;
}
