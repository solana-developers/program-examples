import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Pubkey, type Result, Signer, SystemAccount, SystemProgram, u64 } from '@solanaturbine/poseidon';

export default class PdaRentPayer {
  static PROGRAM_ID = new Pubkey('Db6bufzTcWMDhiCUAuCv3AqCyeZKV4BSGG9ooCibQjrJ');

  initRentVault(payer: Signer, rentVault: SystemAccount, fundLamports: u64): Result {
    rentVault.derive(['rent_vault']);
    SystemProgram.transfer(payer, rentVault, fundLamports);
  }

  createNewAccount(newAccount: Signer, rentVault: SystemAccount, amount: u64): Result {
    rentVault.derive(['rent_vault']);
    SystemProgram.transfer(rentVault, newAccount, amount, ['rent_vault', rentVault.getBump()]);
  }
}
