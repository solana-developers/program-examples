import {
  Pubkey,
  SystemAccount,
  Signer,
  SystemProgram,
  u64,
  type Result,
} from "@solanaturbine/poseidon";

export default class TransferSol {
  static PROGRAM_ID = new Pubkey(
    "BLiyCbPDx54vqpNPQG6A7YAqEM1vRHiFfvReMKC4FFk5"
  );

  // Transferring of SOL using CPI
  transferSolWithCPI(
    payer: Signer, // sender of the SOL
    recipient: SystemAccount, // receiver of transferred SOL
    amount: u64 // amount to be transferred
  ): Result {
    // Invoke the SystemProgram's Transfer instruction
    // Parameters: from, to, amount
    SystemProgram.transfer(payer, recipient, amount);
  }
}
