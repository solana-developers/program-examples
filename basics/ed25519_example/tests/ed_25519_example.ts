import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ed25519Example } from "../target/types/ed25519_example";
import * as ed from "@noble/ed25519";
import { sha512 } from "@noble/hashes/sha512";
import { sha256 } from "@noble/hashes/sha256";
import { randomBytes } from 'tweetnacl';

ed.etc.sha512Sync = (...m) => sha512(ed.etc.concatBytes(...m));


describe("ed25519_example", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ed25519Example as Program<Ed25519Example>;
  const payer = anchor.Wallet.local().payer;

  const admin = anchor.web3.Keypair.generate();
  let msg = new Uint8Array(64);
  msg.set(sha256(randomBytes(64)));

  it("Check signature!", async () => {
    // Add your test here.
    const edSignature = ed.sign(msg, admin.secretKey.slice(0, 32));

    const instruction = await program.methods
      .verifyMessage(Array.from(msg), Array.from(admin.publicKey.toBytes()), Array.from(edSignature))
      .accounts({
        signer: payer.publicKey,
      })
      .instruction();

    let tx = new anchor.web3.Transaction()
      .add(
        // Ed25519 instruction
        anchor.web3.Ed25519Program.createInstructionWithPublicKey({
          publicKey: admin.publicKey.toBytes(),
          message: msg,
          signature: edSignature,
        })
      )
      .add(instruction);
    const { lastValidBlockHeight, blockhash } = await provider.connection.getLatestBlockhash();
    tx.lastValidBlockHeight = lastValidBlockHeight;
    tx.recentBlockhash = blockhash;
    tx.feePayer = payer.publicKey;

    tx.sign(payer, payer);

    const signature = await provider.connection.sendRawTransaction(tx.serialize());

    await provider.connection.confirmTransaction({
      signature,
      blockhash,
      lastValidBlockHeight,
    });
    console.log("Your transaction signature", tx);
  });
});
