import { Account, Pubkey, type Result, u32 } from '@solanaturbine/poseidon';

export default class ProcessingInstructionsProgram {
  static PROGRAM_ID = new Pubkey('FUfFBrs2nHAud8gVESDMtYa7oa5aGa3DEngKKLGyV2hv');

  go_to_park(height: u32, name: String): Result {
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
