import { Pubkey, Result } from "@solanaturbine/poseidon";

export default class HelloWorldProgram {
  static PROGRAM_ID = new Pubkey("6SoSn3xSXpLnJeys6p5ChaoUNdAv7rA4SCdxCanK2zjB");

    helloSolana(): Result {
        console.log("Hello world");
    }

}


  

 