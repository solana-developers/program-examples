import { createCreateTreeInstruction, PROGRAM_ID as BUBBLEGUM_PROGRAM_ID } from "@metaplex-foundation/mpl-bubblegum";
import { loadWalletKey, sendVersionedTx } from "./utils";
import { Connection, Keypair, PublicKey, SystemProgram, Transaction, VersionedMessage } from "@solana/web3.js";
import { SPL_ACCOUNT_COMPRESSION_PROGRAM_ID, SPL_NOOP_PROGRAM_ID, ValidDepthSizePair, getConcurrentMerkleTreeAccountSize } from "@solana/spl-account-compression";
import { SYSTEM_PROGRAM_ID } from "@raydium-io/raydium-sdk";

async function createTree() {
  // Load the wallet key for the user who will create the merkle tree
  const keypair = loadWalletKey("CNFT.json");

  // Create a connection to the network
  const connection = new Connection("https://api.devnet.solana.com");

  // Load the wallet key for the merkle tree account
  const merkleTree = loadWalletKey("TREE.json");

  // Find the tree authority public key and bump seed
  const [treeAuthority, _bump] = PublicKey.findProgramAddressSync(
    [merkleTree.publicKey.toBuffer()],
    BUBBLEGUM_PROGRAM_ID,
  );

  // Define the depth and buffer size of the merkle tree
  const depthSizePair: ValidDepthSizePair = {
    maxDepth: 14,
    maxBufferSize: 64,
  };

  // Calculate the required account space for the merkle tree
  const space = getConcurrentMerkleTreeAccountSize(depthSizePair.maxDepth, depthSizePair.maxBufferSize);

  // Create an account instruction to allocate space for the merkle tree
  const createAccountIx = SystemProgram.createAccount({
    newAccountPubkey: merkleTree.publicKey,
    fromPubkey: keypair.publicKey,
    space: space,
    lamports: await connection.getMinimumBalanceForRentExemption(space),
    programId: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  });

  // Create a merkle tree instruction
  const createTreeIx = createCreateTreeInstruction({
    merkleTree: merkleTree.publicKey,
    treeAuthority: treeAuthority,
    payer: keypair.publicKey,
    treeCreator: keypair.publicKey,
    compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
    logWrapper: SPL_NOOP_PROGRAM_ID,
    systemProgram: SYSTEM_PROGRAM_ID,
  }, {
    maxDepth: depthSizePair.maxDepth,
    maxBufferSize: depthSizePair.maxBufferSize,
    public: false,
  });

  // Send the transaction with both instructions
  const sx = await sendVersionedTx(connection, [createAccountIx, createTreeIx], keypair.publicKey, [keypair, merkleTree]);
  console.log(sx);
}

createTree();
