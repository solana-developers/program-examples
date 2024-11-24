import {
  Account,
  Pubkey,
  Result,
  u64,
  Signer,
  u8,
} from "@solanaturbine/poseidon";

export default class CounterProgram {
  // Define the program ID as a static constant like below
  static PROGRAM_ID = new Pubkey(
    "BmDHboaj1kBUoinJKKSRqKfMeRKJqQqEbUj1VgzeQe4A"
  );

  // Initialize counter state
  initializeCounter(state: CounterState, payer: Signer): Result {
    // Derive PDA using "counter"
    state.derive(["counter"]).init();

    // Initialize the counter to u64(0)
    state.count = new u64(0);
    return { success: true };
  }

  // Increment the counter (similar to upvote)
  increment(state: CounterState): Result {
    state.derive(["counter"]);

    // Increment the counter by 1 using u64 arithmetic
    state.count = state.count.add(1);
    return { success: true };
  }

  // Decrement the counter (similar to downvote)
  decrement(state: CounterState): Result {
    state.derive(["counter"]);

    // Safely decrement the counter by 1 using u64 arithmetic
    state.count = state.count.sub(1);
    return { success: true };
  }
}

// Define custom accounts by creating an interface that extends Account class
export interface CounterState extends Account {
  count: u64; // Counter property with u64 data type
  bump: u8;
}
