import { Pubkey, Result, Signer, SystemAccount, SystemProgram, u64 } from '@solanaturbine/poseidon';

export default class TransferSol {
  // define the progam id as a static constant like bellow
  static PROGRAM_ID = new Pubkey('HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1');

  transferSolWithCpi(payer: Signer, recipient: SystemAccount, amount: u64): Result {
    SystemProgram.transfer(payer, recipient, amount);
  }
}
