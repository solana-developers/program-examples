import { 
  PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID
} from '@metaplex-foundation/mpl-token-metadata';
import * as anchor from "@project-serum/anchor";
import { ASSOCIATED_PROGRAM_ID } from '@project-serum/anchor/dist/cjs/utils/token';
import { NftMinter } from "../target/types/nft_minter";


describe("NFT Minter", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.NftMinter as anchor.Program<NftMinter>;

  const nftTitle = "Homer NFT";
  const nftSymbol = "HOMR";
  const nftUri = "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/nft.json";

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  
  it("Create an NFT!", async () => {

    const metadataAddress = (anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mintKeypair.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
    ))[0];

    const sx = await program.methods.createToken(
      nftTitle, nftSymbol, nftUri
    )
      .accounts({
        metadataAccount: metadataAddress,
        mintAccount: mintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([mintKeypair, payer.payer])
      .rpc();

    console.log("Success!");
        console.log(`   Mint Address: ${mintKeypair.publicKey}`);
        console.log(`   Tx Signature: ${sx}`);
  });

  it("Mint the NFT to your wallet!", async () => {

    const metadataAddress = (anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mintKeypair.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
    ))[0];

    const editionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mintKeypair.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
    ))[0];

    const associatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: payer.publicKey,
    });

    const sx = await program.methods.mintTo()
      .accounts({
        associatedTokenAccount: associatedTokenAccountAddress,
        editionAccount: editionAddress,
        metadataAccount: metadataAddress,
        mintAccount: mintKeypair.publicKey,
        mintAuthority: payer.publicKey,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([payer.payer])
      .rpc();

      console.log("Success!");
      console.log(`   ATA Address: ${associatedTokenAccountAddress}`);
      console.log(`   Tx Signature: ${sx}`);
  });
});
