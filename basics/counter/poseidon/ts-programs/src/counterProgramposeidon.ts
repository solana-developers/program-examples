import { Account, Pubkey, Result, i64, u8, Signer } from "@solanaturbine/poseidon";

export default class CounterProgramPoseidon {
    static PROGRAM_ID = new Pubkey("D5GsmVgazEnZ657NZJS4L6RcmnM8FkhR2qxyxcB4Whb3");

    initializeCounter(counter: CounterState, user: Signer): Result {
        counter.derive(["count"])
            .init();
        // Set the initial value to the `count` field of the account
        counter.count = new i64(0);
    }

    incrementCounter(counter: CounterState): Result {
        counter.derive(["count"]);
        counter.count = counter.count.add(1);
    }

    decrementCounter(counter: CounterState): Result {
        counter.derive(["count"]);
        counter.count = counter.count.sub(1);
    }
}

export interface CounterState extends Account {
    count: i64; // This field store the counter result
    bump: u8; // bump is for PDA (program derieved account, a special type of account which controlled by program on Solana)
}




