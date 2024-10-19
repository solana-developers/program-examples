import {
  Account,
  Pubkey,
  Result,
  u64,
  u8,
  Signer,
} from "@solanaturbine/poseidon";

// Interface representing the state of the counter
export interface CounterState extends Account {
  count: u64; // The current count value
  bump: u8; // A bump value for account derivation
}

export default class CounterProgram {
  // The program ID for the CounterProgram
  static PROGRAM_ID = new Pubkey(
    "DMATyR7jooijeJ2aJYWiyYPf3eoUouumaaLw1JbG3TYF"
  );

  // Method to initialize the counter state
  initialize(state: CounterState, user: Signer): Result {
    state.derive(["count"]).init(); // Derive and initialize the count field
    state.count = new u64(0); // Set the initial count to 0
    return { success: true }; // Return a success result
  }

  // Method to increment the counter state
  increment(state: CounterState): Result {
    state.derive(["count"]); // Derive the count field
    state.count = state.count.add(1); // Increment the count by 1
    return { success: true }; // Return a success result
  }
}
