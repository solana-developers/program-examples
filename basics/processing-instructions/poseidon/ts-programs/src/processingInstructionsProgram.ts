import { Pubkey, Result, u32 } from "@solanaturbine/poseidon";

export default class ProcessingInstructionsProgram {
  static PROGRAM_ID = new Pubkey("6Ra6xuBTuL8ytoHsGEzaAQcHHqNyGjjzemHpFyYrpGdV");

  // for processing instruction in poseidon we need to just write a method that will transpile into anchor instruction

  processingInstructions(height:u32): Result {

    if (Number(height) > 8) {
        console.log(`You can jump from this height length`);
    } else {
       console.log(`you can not jump because you are too short to handle height`);
    };

  }

}
