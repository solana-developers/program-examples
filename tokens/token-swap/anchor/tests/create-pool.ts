import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { SwapExample } from "../target/types/swap_example";
import { TestValues, createValues, expectRevert, mintingTokens } from "./utils";

describe("Create pool", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.SwapExample as Program<SwapExample>;

  let values: TestValues;

  beforeEach(async () => {
    values = createValues();

    await program.methods
      .createAmm(values.id, values.fee)
      .accounts({ amm: values.ammKey, admin: values.admin.publicKey })
      .rpc();

    console.log(values.ammKey);

    await mintingTokens({
      connection,
      creator: values.admin,
      mintAKeypair: values.mintAKeypair,
      mintBKeypair: values.mintBKeypair,
    });
  });

  it("Creation", async () => {
    await program.methods
      .createPool()
      .accounts({
        amm: values.ammKey,
        pool: values.poolKey,
        poolAuthority: values.poolAuthority,
        mintLiquidity: values.mintLiquidity,
        mintA: values.mintAKeypair.publicKey,
        mintB: values.mintBKeypair.publicKey,
        poolAccountA: values.poolAccountA,
        poolAccountB: values.poolAccountB,
      })
      .rpc({ skipPreflight: true });
  });

  it("Invalid mints", async () => {
    values = createValues({
      mintBKeypair: values.mintAKeypair,
      poolKey: PublicKey.findProgramAddressSync(
        [
          values.id.toBuffer(),
          values.mintAKeypair.publicKey.toBuffer(),
          values.mintBKeypair.publicKey.toBuffer(),
        ],
        program.programId
      )[0],
      poolAuthority: PublicKey.findProgramAddressSync(
        [
          values.id.toBuffer(),
          values.mintAKeypair.publicKey.toBuffer(),
          values.mintBKeypair.publicKey.toBuffer(),
          Buffer.from("authority"),
        ],
        program.programId
      )[0],
    });

    await expectRevert(
      program.methods
        .createPool()
        .accounts({
          amm: values.ammKey,
          pool: values.poolKey,
          poolAuthority: values.poolAuthority,
          mintLiquidity: values.mintLiquidity,
          mintA: values.mintAKeypair.publicKey,
          mintB: values.mintBKeypair.publicKey,
          poolAccountA: values.poolAccountA,
          poolAccountB: values.poolAccountB,
        })
        .rpc()
    );
  });
});
