import { Account, Pubkey, Result, i64, u8, Signer } from "@solanaturbine/poseidon";

export default class FavoritesProgram {
    static PROGRAM_ID = new Pubkey("9Ewc72Ju79bJ3qEaASXtNjAQphdJcHCNpELcwTFVtXnM");

    initialize(state: FavoriteState, user: Signer): Result {
        state.derive(["favorites"])
            .init()

        state.favorite = new i64(0)
    }

    add(state: FavoriteState): Result {
        state.derive(["favorites"])
        state.favorite = state.favorite.add(1)
    }

    remove(state: FavoriteState): Result {
        state.derive(["favorites"])
        state.favorite = state.favorite.sub(1)
    }
}

export interface FavoriteState extends Account {
    favorite: i64
    bump: u8
}