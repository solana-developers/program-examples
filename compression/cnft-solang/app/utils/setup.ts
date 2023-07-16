import { Program, Idl } from "@coral-xyz/anchor";
import { IDL, CompressedNft } from "../idl/compressed_nft";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";

export const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

const programId = IDL.metadata.address;

export const program = new Program(IDL as Idl, programId, {
  connection,
}) as unknown as Program<CompressedNft>;

export const treeAddress = new PublicKey(
  "FYwZ4rMtexsHBTx2aCnQRVg51K5WXZJ1n3SYacXFvado"
);
