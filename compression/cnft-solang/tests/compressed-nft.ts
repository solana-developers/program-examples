import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { CompressedNft } from "../target/types/compressed_nft";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  Keypair,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  ValidDepthSizePair,
  SPL_NOOP_PROGRAM_ID,
  createAllocTreeIx,
} from "@solana/spl-account-compression";
import {
  PROGRAM_ID as BUBBLEGUM_PROGRAM_ID,
  createCreateTreeInstruction,
} from "@metaplex-foundation/mpl-bubblegum";
import { uris } from "../utils/uri";

describe("compressed-nft", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as Wallet;
  const connection = provider.connection;

  const program = anchor.workspace.CompressedNft as Program<CompressedNft>;

  // Generate a new keypair for the merkle tree.
  const treeKeypair = Keypair.generate();

  // Derive the PDA that will be the tree authority.
  // This is required by the bubblegum program.
  const [treeAuthority] = PublicKey.findProgramAddressSync(
    [treeKeypair.publicKey.toBuffer()],
    BUBBLEGUM_PROGRAM_ID
  );

  // Derive the PDA that will be used to initialize the dataAccount.
  // Required by Solang even though we're not using it.
  const [dataAccount, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("seed")],
    program.programId
  );

  // Create a merkle tree account.
  before(async () => {
    // Maximum depth and buffer size for the merkle tree.
    // 2^maxDepth determines the maximum number of leaves that can be stored in the tree.
    // maxBufferSize determines maximum concurrent updates that can be made within one slot.
    const maxDepthSizePair: ValidDepthSizePair = {
      maxDepth: 14,
      maxBufferSize: 64,
    };

    // Depth of the canopy (how much of the tree is stored on-chain)
    const canopyDepth = 0;

    // Instruction to create an account with enough space to store the merkle tree.
    const allocTreeIx = await createAllocTreeIx(
      connection,
      treeKeypair.publicKey,
      wallet.publicKey,
      maxDepthSizePair,
      canopyDepth
    );

    // Instruction to initialize the merkle tree account with the bubblegum program.
    const createTreeIx = createCreateTreeInstruction(
      {
        treeAuthority,
        merkleTree: treeKeypair.publicKey,
        payer: wallet.publicKey,
        treeCreator: wallet.publicKey,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
      },
      {
        maxBufferSize: maxDepthSizePair.maxBufferSize,
        maxDepth: maxDepthSizePair.maxDepth,
        public: true, // creating a "public" tree, so anyone can mint cnfts to it
      },
      BUBBLEGUM_PROGRAM_ID
    );

    try {
      const tx = new Transaction().add(allocTreeIx, createTreeIx);
      tx.feePayer = wallet.publicKey;

      const txSignature = await sendAndConfirmTransaction(
        connection,
        tx,
        [treeKeypair, wallet.payer],
        {
          commitment: "confirmed",
          skipPreflight: true,
        }
      );

      console.log(
        `https://explorer.solana.com/tx/${txSignature}?cluster=devnet`
      );

      console.log("Tree Address:", treeKeypair.publicKey.toBase58());
    } catch (err: any) {
      console.error("\nFailed to create merkle tree:", err);
      throw err;
    }

    console.log("\n");
  });

  it("Is initialized!", async () => {
    // Initialize the dataAccount.
    const tx = await program.methods
      .new([bump])
      .accounts({ dataAccount: dataAccount })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Mint Compressed NFT", async () => {
    // Mint a compressed nft to random receiver.
    const receiver = Keypair.generate().publicKey;

    // Use a random uri (off-chain metadata) from the list for the test.
    const randomUri = uris[Math.floor(Math.random() * uris.length)];

    const tx = await program.methods
      .mint(
        treeAuthority, // treeAuthority
        receiver, // leafOwner
        receiver, // leafDelegate
        treeKeypair.publicKey, // merkleTree
        wallet.publicKey, // payer
        wallet.publicKey, // treeDelegate
        randomUri // uri
      )
      .accounts({ dataAccount: dataAccount }) // dataAccount required by Solang even though its unused.
      .remainingAccounts([
        {
          pubkey: wallet.publicKey, // payer (and tree delegate in this example)
          isWritable: true,
          isSigner: true,
        },
        {
          pubkey: receiver, // new leaf owner
          isWritable: false,
          isSigner: false,
        },
        {
          pubkey: treeAuthority, // tree authority
          isWritable: true,
          isSigner: false,
        },
        {
          pubkey: treeKeypair.publicKey, // tree account address
          isWritable: true,
          isSigner: false,
        },
        {
          pubkey: SPL_NOOP_PROGRAM_ID,
          isWritable: false,
          isSigner: false,
        },
        {
          pubkey: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
          isWritable: false,
          isSigner: false,
        },
        {
          pubkey: BUBBLEGUM_PROGRAM_ID,
          isWritable: false,
          isSigner: false,
        },
        {
          pubkey: SystemProgram.programId,
          isWritable: false,
          isSigner: false,
        },
      ])
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });
});
