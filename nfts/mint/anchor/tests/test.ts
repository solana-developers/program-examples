import * as anchor from "@project-serum/anchor";
import { MintNft } from "../target/types/mint_nft";


const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const testTokenTitle = "Solana Platinum NFT";
const testTokenSymbol = "SOLP";
const testTokenUri = "https://raw.githubusercontent.com/solana-developers/program-examples/main/nfts/nft_metadata.json";


describe("mint-token", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.MintNft as anchor.Program<MintNft>;

  it("Mint!", async () => {

    const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    console.log(`New token: ${mintKeypair.publicKey}`);

    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];

    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: payer.publicKey
    });

    await program.methods.mintToken(
      testTokenTitle, testTokenSymbol, testTokenUri
    )
    .accounts({
      metadataAccount: metadataAddress,
      mintAccount: mintKeypair.publicKey,
      tokenAccount: tokenAddress,
      mintAuthority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([payer.payer, mintKeypair])
    .rpc();
  });
});
