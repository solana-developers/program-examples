import { Pubkey, Result, Signer, u32 } from '@solanaturbine/poseidon';

export default class ProcessingInstructions {
  static PROGRAM_ID = new Pubkey('ESFoo5N4Zv65pph6thqP3HVqiY7KH5o5V8TqTsnmB2vw');

  goToPark(user: Signer, name: string): Result {
    console.log(`Welcome to the park, ${name}!`);
  }
}
