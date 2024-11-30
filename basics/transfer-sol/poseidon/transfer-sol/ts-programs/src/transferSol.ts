import {
  Pubkey,
  Signer,
  SystemAccount,
  SystemProgram,
  u64,
  type Result,
} from "@solanaturbine/poseidon";

export default class TransferSol {
  static PROGRAM_ID = new Pubkey(
    "97hdpUKsrwzwkThN7hiySbssDRujp3kjFnKEpuSD66uk"
  );

  transferSol(
    payer: Signer,
    recipient: SystemAccount,
    transferAmount: u64
  ): Result {
    SystemProgram.transfer(payer, recipient, transferAmount);
  }
}
