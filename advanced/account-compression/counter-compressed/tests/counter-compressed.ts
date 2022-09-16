import * as anchor from "@project-serum/anchor";
import { Program, BN, ACCOUNT_DISCRIMINATOR_SIZE } from "@project-serum/anchor";
import { CounterCompressed } from "../target/types/counter_compressed";
import { PublicKey, Keypair, SystemProgram } from '@solana/web3.js';

import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
  getConcurrentMerkleTreeAccountSize,
  getConcurrentMerkleTree,
  getCMTCurrentRoot,
} from '@ngundotra/spl-account-compression';

import {
  programSupportsExtensions,
  TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import { assert } from "chai";

describe("counter-compressed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CounterCompressed as Program<CounterCompressed>;

  let payer: PublicKey = program.provider.publicKey;

  // Merkle tree id
  let merkleTreeKeypair = Keypair.generate();
  let merkleTree = merkleTreeKeypair.publicKey;

  // Calculate id of the counter's pubkey when decompressed
  const [counter, _bump] = PublicKey.findProgramAddressSync(
    [merkleTree.toBuffer(), Buffer.from("counter"), new BN(0).toBuffer('le', 4)],
    program.programId
  );

  const maxDepth = 3;
  const maxBufferSize = 8;
  const canopyDepth = 3;
  let merkleTreeSpace = getConcurrentMerkleTreeAccountSize(maxDepth, maxBufferSize, canopyDepth);
  console.log(program.programId.toString());

  it("Can initialize a global counter tree", async () => {
    const tx = await program.methods
      .initializeGlobalCounterTree(maxDepth, maxBufferSize)
      .accounts({
        payer,
        merkleTree,
        accountCompressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        splNoopProgram: SPL_NOOP_PROGRAM_ID,
      })
      .preInstructions([
        SystemProgram.createAccount({
          fromPubkey: payer,
          newAccountPubkey: merkleTree,
          lamports: await program.provider.connection.getMinimumBalanceForRentExemption(merkleTreeSpace, "confirmed"),
          space: merkleTreeSpace,
          programId: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        })
      ])
      .signers([merkleTreeKeypair])
      .rpc();
    // console.log("Your transaction signature", tx);
  });
  it("Can initialize compressed counter", async () => {
    const tx = await program.methods
      .initializeCompressedCounter()
      .accounts({
        merkleTree,
        accountCompressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        splNoopProgram: SPL_NOOP_PROGRAM_ID,
      })
      .rpc({ skipPreflight: true })
    // console.log("Init compressed counter", tx);
  });
  it("Can increment compressed counter", async () => {
    const cmt = await getConcurrentMerkleTree(program.provider.connection, merkleTree);
    const root = getCMTCurrentRoot(cmt);

    const tx = await program.methods
      .incrementCompressedCounter({
        tree: merkleTree,
        id: 0,
        count: new BN(0),
      }, Array.from(root))
      .accounts({
        merkleTree,
        accountCompressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        splNoopProgram: SPL_NOOP_PROGRAM_ID,
      })
      .rpc({ skipPreflight: true })
  });
  it("Can decompress compressed counter", async () => {
    const cmt = await getConcurrentMerkleTree(program.provider.connection, merkleTree);
    const root = getCMTCurrentRoot(cmt);

    const tx = await program
      .methods
      .decompressCounter({
        tree: merkleTree,
        id: 0,
        count: new BN(1)
      }, Array.from(root))
      .accounts({
        payer,
        merkleTree,
        counter,
        accountCompressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        splNoopProgram: SPL_NOOP_PROGRAM_ID,
      })
      .rpc();

    const counterInfo = await program.account.counter.fetch(counter);
    assert(counterInfo.count.eq(new BN(1)), "Decompressed counter is wrong")
    assert(counterInfo.id === 0, "Decompressed counter has incorrect id");
    assert(counterInfo.tree.equals(merkleTree), "Decompressed counter has incorrect tree id");
    // console.log("Decompress tx", tx);
  });
  it("Can increment decompressed counter", async () => {
    const tx = await program
      .methods
      .incrementCounter()
      .accounts({
        counter,
      })
      .rpc();
    const counterInfo = await program.account.counter.fetch(counter);
    assert(counterInfo.count.eq(new BN(2)), "Decompressed counter did not increment");
    console.log("Increment decompressed counter:", tx);
  });
  it("Can compress existing counter", async () => {
    const cmt = await getConcurrentMerkleTree(program.provider.connection, merkleTree);
    const root = getCMTCurrentRoot(cmt);

    const tx = await program
      .methods
      .compressCounter(Array.from(root))
      .accounts({
        claimer: payer,
        counter,
        merkleTree,
        accountCompressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        splNoopProgram: SPL_NOOP_PROGRAM_ID
      })
      .rpc()

    console.log("Compress counter:", tx);
    console.log('\tclosed:', counter.toBase58());
  });
});
