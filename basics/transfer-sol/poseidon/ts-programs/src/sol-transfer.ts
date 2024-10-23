import {
  Constraint,
  Pubkey,
  type Result,
  Signer,
  SystemAccount,
  SystemProgram,
  u64,
  UncheckedAccount,
} from "@solanaturbine/poseidon";

export default class TransferSol {
  static PROGRAM_ID = new Pubkey(
    "DwKzVHWsZEUKrySDHQwFW1J8nFWGtz7HzjY9FY55PMDS"
  );

  transferSolWithCpi(
    payer: Signer,
    recipient: SystemAccount,
    amount: u64
  ): Result {
    SystemProgram.transfer(payer, recipient, amount);
  }

  // Directly modifying lamports is only possible if the program is the owner of the account
  // Fails if the program is not the owner of the payer account
  transferSolWithProgram(
    payer: UncheckedAccount,
    recipient: SystemAccount,
    amount: u64
  ): Result {
    payer.is_writable = true;
    recipient.is_writable = true;
    // Note: No method to substract u64 from another u64
    // payer.lamports = payer.lamports.sub(amount);
    // recipient.lamports = recipient.lamports.add(amount);
  }
}
