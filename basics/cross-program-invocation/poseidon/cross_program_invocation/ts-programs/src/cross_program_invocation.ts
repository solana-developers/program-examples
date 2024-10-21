import {
  Account,
  Pubkey,
  Result,
  Signer,
} from "@solanaturbine/poseidon";

export default class CrossProgramInvocation {
  static PROGRAM_ID = new Pubkey(
    "D4aA71us8bTcdXeZQpXyXidW2xPugVwUuoXx3b1bnvXa"
  );

  initialize() {}

  switchPower(name:String) {}
  pullLever(
    user: Signer,
    power: PowerStatus,
    lever_program: Lever,
    name: String
  ): Result {}

  initializeLever(user: Signer, power: PowerStatus): Result {
    power.derive(["power"]).init();
  }

  setPowerStatus(user: Signer, power: PowerStatus): Result {}
}

export interface PowerStatus extends Account {
  // is_on: bool
}

export interface Lever extends Account {}
