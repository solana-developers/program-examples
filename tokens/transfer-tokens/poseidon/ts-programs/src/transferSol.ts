import { Pubkey, Result, Signer, SystemAccount, SystemProgram, u64 } from '@solanaturbine/poseidon';

export default class TransferSol {
  static PROGRAM_ID = new Pubkey('HU7QEokj5qUUV5ryZYL7EhsqiAkxJyMVXc3DesKyCqtF');

  transferWithProgram(from: Signer, to: SystemAccount, amount: u64): Result {
    SystemProgram.transfer(from, to, amount);
  }
}
