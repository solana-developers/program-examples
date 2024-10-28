import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Favorites as Program<Favorites>;

  // Generate new user keypairs for testing
  const user = Keypair.generate();

  // Variables for storing PDA and bump
  let favoritesPDA: PublicKey;
  let favoritesBump: number;

  // Define the favorites data
  const favoritesInfo = {
    number: new anchor.BN(23),
    color: "purple",
    hobbbies: ["skiing", "skydiving", "biking"],
  };

  before(async () => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    // Airdrop 1 SOL to the user that will be used for testing
    const airdropUser = await provider.connection.requestAirdrop(
      user.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropUser,
    });

    // Derive PDA for the favorites account
    [favoritesPDA, favoritesBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Creates a Favorites account", async () => {
    // Invoke the SetFavorites Instruction from the program
    // Parameters: number: u64, color: String, hobbies: Vec<String>,
    await program.methods
      .setFavorites(
        favoritesInfo.number,
        favoritesInfo.color,
        favoritesInfo.hobbbies
      )
      .accountsPartial({
        user: user.publicKey,
        favorites: favoritesPDA,
      })
      .signers([user])
      .rpc();

    // Fetch the favorites account info
    const favoritesAccountInfo = await program.account.favoritesAccount.fetch(
      favoritesPDA
    );

    // Assertion and comparison of the proper values
    assert.equal(
      favoritesAccountInfo.number.toNumber(),
      favoritesInfo.number.toNumber(),
      "Favorite number doesn't match"
    );
    assert.equal(
      favoritesAccountInfo.color,
      favoritesInfo.color,
      "Favorite color doesn't match"
    );
    favoritesInfo.hobbbies.forEach(function (hobby) {
      assert.include(
        favoritesAccountInfo.hobbies,
        hobby,
        "Favorite hobbies doesn't match"
      );
    });
  });

  it("Updates the favorite hobbies in the Favorites account", async () => {
    // Define a new favorite hobbies
    const newfavoritesHobbies = ["skiing", "skydiving", "biking", "swimming"];

    await program.methods
      .setFavorites(
        favoritesInfo.number,
        favoritesInfo.color,
        newfavoritesHobbies  // replaces favoritesInfo.hobbies with newFavoriteHobbies
      )
      .accountsPartial({
        user: user.publicKey,
        favorites: favoritesPDA,
      })
      .signers([user])
      .rpc();

    // Fetch the favorites account info
    const favoritesAccountInfo = await program.account.favoritesAccount.fetch(
      favoritesPDA
    );

    // Assertion and comparison of the proper values
    assert.equal(
      favoritesAccountInfo.number.toNumber(),
      favoritesInfo.number.toNumber(),
      "Favorite number doesn't match"
    );
    assert.equal(
      favoritesAccountInfo.color,
      favoritesInfo.color,
      "Favorite color doesn't match"
    );
    newfavoritesHobbies.forEach(function (hobby) {
      assert.include(
        favoritesAccountInfo.hobbies,
        hobby,
        "Favorite hobbies doesn't match"
      );
    });
  });
});
