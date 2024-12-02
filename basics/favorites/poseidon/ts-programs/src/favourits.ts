import {
  Account,
  Pubkey,
  Signer,
  u8,
  type Result,
  // Avoid using `as` for renaming imports in this case, as it can cause issues during the transpilation process.
  String,
  Vec,
} from "@solanaturbine/poseidon";

// This program defines a Favorites class that allows a Solana user to store their favorite number.
// It uses Program Derived Addresses (PDAs) to securely manage the user's favorite data.

export default class Favorites {
  static PROGRAM_ID = new Pubkey(
    "GsGBeoB6fFTWfUrHhKYTjtXvuiKCC7shhhQqXeQsTLJ2"
  );

  initialize(
    state: FavoritesState,
    user: Signer,
    number: u8,
    color: String<10>,
    hobbies: Vec<String<10>, 5>
  ): Result {
    // Create a PDA using the seed combination ["favorites", user.key]
    state.derive(["favorites", user.key]).init(user);

    state.owner = user.key; // Set the owner of the favorites state to the user's public key
    state.number = number; // Store the user's favorite number
    state.color = color;
    state.hobbies = hobbies;
    state.bump = state.getBump(); // Retrieve the bump seed for the PDA
  }
}

export interface FavoritesState extends Account {
  owner: Pubkey; // Public key of the account owner

  // PDA properties
  number: u8; // The user's favorite number
  color: String<10>;
  hobbies: Vec<String<10>, 5>;
  bump: u8; // Bump seed for the PDA
}
