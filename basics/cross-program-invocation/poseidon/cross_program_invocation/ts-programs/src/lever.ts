import { Account, Pubkey, Result, Signer } from '@solanaturbine/poseidon';

export default class Lever {
  static PROGRAM_ID = new Pubkey('9aM9w7ozrZwXx9bQHbBx6QjWc6F46tdN9ayt86vt9uLL');

  initialize(user: Signer, power: Signer) {}

  //   switchPower(name: String) {}

  initializeLever(user: Signer, power: PowerStatus): Result {
    power.derive(['lever']).init();
  }

  setPowerStatus(user: Signer, power: PowerStatus): Result {}
}

export interface PowerStatus extends Account {
  // is_on: bool
}
