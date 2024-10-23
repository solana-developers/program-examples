import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Favorites } from "../target/types/favorites";
import { expect } from "chai";

describe("account favorites program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Favorites as Program<Favorites>;

  const favoritesAccount = new Keypair();
  const [pdaAccount, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("favorites"), favoritesAccount.publicKey.toBuffer()],
    program.programId
  );

  it("Initialize account favorites", async () => {
    console.log(`Favorites Account: ${favoritesAccount.publicKey}`);

    // Request an AirDrop
    const airdropSignature = await provider.connection.requestAirdrop(
      favoritesAccount.publicKey,
      1 * LAMPORTS_PER_SOL // Request 1 SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);

    const favorites = {
      number: 2,
      // color: "blue",
    };
    await program.methods
      .initialize(favorites.number)
      .accounts({
        user: favoritesAccount.publicKey,
      })
      .signers([favoritesAccount])
      .rpc();

    const account = await program.account.favoritesState.fetch(pdaAccount);
    console.log(`Owner: ${account.owner}`);
    console.log(`Number: ${account.number}`);
    console.log(`Bump: ${account.bump}`);
    // console.log(`Color: ${account.color}`);

    expect(account.bump).to.be.a("number");
    expect(account.owner.toBase58()).to.equal(
      favoritesAccount.publicKey.toBase58()
    );
    expect(account.number).to.equal(favorites.number);
  });
});
