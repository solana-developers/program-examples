import { Account, Pubkey, type Result, i64, u8, Signer } from "@solanaturbine/poseidon";

export default class CounterProgram {
    static PROGRAM_ID = new Pubkey("36PvqMz57YD68SfLBTLL9bYhQdw1BdL4kGKK3krdVoSA");

    initialize_counter(state: CounterState, user: Signer): Result {

        state.derive(["count"])
            .init();

        state.count = new i64(0);
    }

    increment(state: CounterState): Result {
        state.derive(["count"]);
        state.count = state.count.add(1);
    }

    decrement(state: CounterState): Result {
        state.derive(["count"]);
        state.count = state.count.sub(1);
    }
}

export interface CounterState extends Account {
    count: i64;
    bump: u8;
}