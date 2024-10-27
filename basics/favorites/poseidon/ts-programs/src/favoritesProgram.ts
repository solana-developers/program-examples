// biome-ignore lint/suspicious/noShadowRestrictedNames: Solana Turbine explicitly uses String as a type name
import { Account, Pubkey, Result, Signer, String, u8, u64 } from '@solanaturbine/poseidon';

export default class FavoritesProgram {
  static PROGRAM_ID = new Pubkey('HMYL9ABJz8fpw6XUnkRAYVsXor4JxosiZqHBd38ZgCqS');

  setFavorites(favorites: Favorites, payer: Signer, number: u64, color: String, hobbies: String[]): Result {
    favorites.derive(['favorites', payer.key]).init();
    favorites.number = number;
    favorites.color = color;
    favorites.hobbies = hobbies;
    favorites.bump = favorites.getBump();
  }
}

export interface Favorites extends Account {
  number: u64;
  color: String;
  hobbies: String[];
  bump: u8;
}
