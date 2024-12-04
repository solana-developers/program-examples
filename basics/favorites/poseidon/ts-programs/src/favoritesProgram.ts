import { Account, Pubkey, Result, Signer, Str, Vec, u8, u64 } from '@solanaturbine/poseidon';

export default class FavoritesProgram {
  static PROGRAM_ID = new Pubkey('BreVFi2U3pUegY96xP5JMviUuxL5x6bRnnbsztb262vQ');

  setFavorites(favorites: Favorites, payer: Signer, number: u64, color: Str<7>, hobbies: Vec<Str<7>, 5>): Result {
    favorites.derive(['favorites', payer.key]).init(payer);
    favorites.number = number;
    favorites.color = color;
    favorites.hobbies = hobbies;
    favorites.bump = favorites.getBump();
  }
}

export interface Favorites extends Account {
  number: u64;
  color: Str<7>;
  hobbies: Vec<Str<7>, 5>;
  bump: u8;
}
