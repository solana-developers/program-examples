import { Pubkey, type Result } from '@solanaturbine/poseidon';

export default class Poseidon {
  static PROGRAM_ID = new Pubkey('7fkznAWPGcMC1ck9Y1c16gMh8wqf1J4NgyJ9NFzGfRaV');

  hello(): Result {
    console.log('Hello, Solana!');
    console.log(`Program ID: ${Poseidon.PROGRAM_ID.toString()}`);
    return {
      accounts: [],
      instructions: [],
      signers: [],
    };
  }
}
