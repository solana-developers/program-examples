import { Pubkey, Result } from "@solanaturbine/poseidon";

export default class HelloSolanaProgram {
  static PROGRAM_ID = new Pubkey(
    "2phbC62wekpw95XuBk4i1KX4uA8zBUWmYbiTMhicSuBV"
  );

  // Define the main 'hello' function similar to the Rust function
  hello(): Result {
    // Equivalent of Solana's `msg!` for logging
    console.log("Hello, Solana!");

    // Log the program ID as well
    console.log(`Our program's Program ID: ${HelloSolanaProgram.PROGRAM_ID}`);
  }
}
