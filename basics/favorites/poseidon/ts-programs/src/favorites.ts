import {
  Pubkey,
  type Result,
  Signer,
  u64,
  String,
  Account,
  Vec,
} from "@solanaturbine/poseidon";

export default class Favorites {
  static PROGRAM_ID = new Pubkey(
    "H4mwUctvTzbQ7bgxceko6eoi3qYH9vmFqwMoPVQ9vf5T"
  );

  // SetFavorite Instruction
  setFavorites(
    // ACCOUNTS

    user: Signer,
    favorites: FavoritesAccount,
    number: u64,
    color: String<50>,
    hobbies: Vec<String<50>, 5>
  ): Result {
    // CONTEXT

    // .derive() ensures that the account is a PDA derived from the parameters as it seed
    // .initIfNeeded() ensures that the init_if_needed constraint will be used for the account initialization
    favorites.derive(["favorites", user.key]).initIfNeeded();

    // Set the FavoritesAccount State variables to the one inputted by the user
    favorites.number = number;
    favorites.color = color;
    favorites.hobbies = hobbies;
  }
}

// STATES
export interface FavoritesAccount extends Account {
  number: u64;
  color: String<50>;
  hobbies: Vec<String<50>, 5>;
}
