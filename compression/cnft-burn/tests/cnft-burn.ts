import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CnftBurn } from "../target/types/cnft_burn";
import {
  MPL_BUBBLEGUM_PROGRAM_ID,
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
} from "@metaplex-foundation/mpl-bubblegum";
import { decode, mapProof } from "./utils";
import { getAsset, getAssetProof } from "./readApi";

describe("cnft-burn", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CnftBurn as Program<CnftBurn>;
  const provider = anchor.AnchorProvider.env();
  const payerWallet = provider.wallet as anchor.Wallet;
  // this should be your tree address
  const tree = new anchor.web3.PublicKey(
    "Dggp3P5C7rB5crU3TnWMGYYKTy1At1dzwE5Ax9Sz46Kj"
  );
  const MPL_BUBBLEGUM_PROGRAM_ID_KEY = new anchor.web3.PublicKey(
    MPL_BUBBLEGUM_PROGRAM_ID
  );
  const [treeAuthority, _bump2] = anchor.web3.PublicKey.findProgramAddressSync(
    [tree.toBuffer()],
    MPL_BUBBLEGUM_PROGRAM_ID_KEY
  );
  console.log("Tree Authority", treeAuthority.toString());
  console.log(
    "Computed tree authority",
    "2zhktLCwGLFg6bqGxgdN5BEKT7PVsQ81XyfQ33gKVtxU"
  );
  // this is the assetId of the cNft you want to burn
  const assetId = "2joTFxoKshsWXT2QAdjZVdvqVmGv6FhTZ2s5TCCYz7Eo";

  it("Burn cNft!", async () => {
    const asset = await getAsset(assetId);

    const proof = await getAssetProof(assetId);
    const proofPathAsAccounts = mapProof(proof);
    const root = decode(proof.root);
    const dataHash = decode(asset.compression.data_hash);
    const creatorHash = decode(asset.compression.creator_hash);
    const nonce = new anchor.BN(asset.compression.leaf_id);
    const index = asset.compression.leaf_id;
    const tx = await program.methods
      .burnCnft(root, dataHash, creatorHash, nonce, index)
      .accounts({
        merkleTree: tree,
        leafOwner: payerWallet.publicKey,
        treeAuthority: treeAuthority,

        bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .remainingAccounts(proofPathAsAccounts)
      .rpc({
        skipPreflight: true,
      });
    console.log("Your transaction signature", tx);
  });
});
