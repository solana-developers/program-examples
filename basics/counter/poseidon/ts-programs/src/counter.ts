import { Account, Pubkey, Signer, u64, type Result } from "@solanaturbine/poseidon";


export default class Counter {
    static PROGRAM_ID = new Pubkey("GnL9WWgvnFbhvNedx6LTdPt4QeWXM4XdAtnRE4uToXdV");

    // Initialize Counter instruction
    initializeCounter(
        // ACCOUNTS

        payer: Signer, // user paying for the counter account creation
        counter: CounterAccount, 
    ): Result {
        // CONTEXT

        // .derive([<seeds>]) ensures that account is a PDA with the <seeds> as the parameters
        // .init() ensures that the account will have the init constraint when transpiled
        counter.derive(["counter"]).init();

        // Assign the initial value of the counter to 0
        counter.count = new u64(0);
    }

    // Increment Counter Instruction
    incrementCounter(
        // ACCOUNTS

        payer: Signer, // user payingfor incrementing the counter account
        counter: CounterAccount,
    ): Result {
        // CONTEXT

        counter.derive(["counter"]);

        // Increment the counter by 1
        counter.count = counter.count.add(1);
    }
}

// STATE
export interface CounterAccount extends Account {
    count: u64
}