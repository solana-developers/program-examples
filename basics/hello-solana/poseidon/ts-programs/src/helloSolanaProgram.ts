import { Pubkey, Result } from "@solanaturbine/poseidon";

export default class HelloSolanaProgram {
  static PROGRAM_ID = new Pubkey(
    "DaNK9CdncCbPrHRWJpWL1oyEBS9M985YYXR8WTQzYSdE"
  );

  helloSolana(): Result {
    console.log("Hello, Solana!");
  }
}