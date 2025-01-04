import { Pubkey, Result } from '@solanaturbine/poseidon';

export default class HelloSolanaProgram {
  static PROGRAM_ID = new Pubkey('2phbC62wekpw95XuBk4i1KX4uA8zBUWmYbiTMhicSuBV');

  hello(): Result {
    console.log('Hello, Solana!');

    console.log(`Our program's Program ID: ${HelloSolanaProgram.PROGRAM_ID}`);
  }
}
