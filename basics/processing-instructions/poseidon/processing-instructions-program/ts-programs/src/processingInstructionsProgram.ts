import { Account, Pubkey, type Result, String, u32 } from '@solanaturbine/poseidon';

export default class ProcessingInstructionsProgram {
  static PROGRAM_ID = new Pubkey('3tmL8d7yC7cRDwQ1B8rcsY6YzWTGVz65aw1ooH2YbnME');

  go_to_park(height: u32, name: String<25>): Result {
    // Display a welcome message
    console.log('Welcome to the park,', name);
    // Check if the height is above the threshold
    if (Number(height) > 5) {
      console.log('You are tall enough to ride this ride. Congratulations.');
    } else {
      console.log('You are NOT tall enough to ride this ride. Sorry mate.');
    }
  }
}
