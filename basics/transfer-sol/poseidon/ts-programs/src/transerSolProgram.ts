import { Pubkey, Result, u64, Signer, SystemAccount, SystemProgram } from "@solanaturbine/poseidon";

export default class TransferSolProgram {
  static PROGRAM_ID = new Pubkey("7VjyAirb4LLbGGTBqzCuYqeirue9S9Zj2fDfUYVU4YdA");

  sendSol(
    sender: Signer,
    receiver: SystemAccount,
    amount: u64,
  ): Result {

    SystemProgram.transfer(
        sender,
        receiver,
        amount
    )

  }
}