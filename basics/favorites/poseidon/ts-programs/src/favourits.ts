import {
  Account,
  Pubkey,
  Signer,
  u8,
  type Result,
} from "@solanaturbine/poseidon";

// * This program defines a Favorites class that allows a Solana user to store their favorite number.
// * It uses Program Derived Addresses (PDAs) to securely manage the user's favorite data.

export default class Favorites {
  static PROGRAM_ID = new Pubkey(
    "GsGBeoB6fFTWfUrHhKYTjtXvuiKCC7shhhQqXeQsTLJ2"
  );

  // Note: As of this implementation, the Poseidon framework does not support using arrays of strings
  // (e.g., String[] or Array<String>) in the transaction context. Therefore,
  // it is not possible to use collections of strings, such as hobbies: String[] or Array<String>.

  // Additionally, working with string data types can lead to deserialization issues,
  // specifically the `AccountDidNotDeserialize` error. This occurs because the framework may
  // not correctly allocate the required space for PDAs that include string fields.

  initialize(
    user: Signer,
    state: FavoritesState,
    number: u8
    // color: String,
    // hobbies: Array<String>
  ): Result {
    // Create a PDA using the seed combination ["favorites", user.key]
    state.derive(["favorites", user.key]).init();

    state.owner = user.key; // Set the owner of the favorites state to the user's public key
    state.number = number; // Store the user's favorite number
    // state.color = color; // Note: String handling is currently unsupported
    // state.hobbies = hobbies; //  Note: Array of String is currently unsupported
    state.bump = state.getBump(); // Retrieve the bump seed for the PDA
  }
}

export interface FavoritesState extends Account {
  owner: Pubkey; // Public key of the account owner

  // PDA properties
  number: u8; // The user's favorite number
  // color: String; // Note: String handling is currently unsupported
  // hobbies: Array<String>; // Note: String handling is currently unsupported
  bump: u8; // Bump seed for the PDA
}
