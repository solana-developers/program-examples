import {
  Pubkey,
  Result,
  Signer,
  SystemAccount,
  SystemProgram,
  u64,
} from "@solanaturbine/poseidon";

export default class TransferSolProgram {
  static PROGRAM_ID = new Pubkey("11111111111111111111111111111111");

  // Transfer SOL using SystemProgram CPI
  transferSol(
    sender: Signer,
    recipient: SystemAccount,
    amount: u64
  ): Result {
    SystemProgram.transfer(
      sender,
      recipient,
      amount
    );
  }
}
