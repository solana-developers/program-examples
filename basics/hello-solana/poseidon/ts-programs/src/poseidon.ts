import { Pubkey, type Result } from "@solanaturbine/poseidon";

export default class HelloSolana {
    static PROGRAM_ID = new Pubkey("9zexDtgqhQvcMkCrZDco2oP4B5cyhmxMttt8aN52g6CP");

    hello(): Result {
        console.log("Hello, Solana!")

        console.log(`Our program's Program ID: ${HelloSolana.PROGRAM_ID.toString()}`)
    }
}