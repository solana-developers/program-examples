import { NextApiRequest, NextApiResponse } from "next"
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js"
import {
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
} from "@solana/spl-account-compression"
import { PROGRAM_ID as BUBBLEGUM_PROGRAM_ID } from "@metaplex-foundation/mpl-bubblegum"
import { program, connection, treeAddress } from "@/utils/setup"
import { uris } from "@/utils/uri"

function get(res: NextApiResponse) {
  res.status(200).json({
    label: "My Store",
    icon: "https://solana.com/src/img/branding/solanaLogoMark.svg",
  })
}

async function post(req: NextApiRequest, res: NextApiResponse) {
  const { account } = req.body
  const { reference } = req.query

  if (!account || !reference) {
    res.status(400).json({
      error: "Required data missing. Account or reference not provided.",
    })
    return
  }

  try {
    const transaction = await buildTransaction(
      new PublicKey(account),
      new PublicKey(reference)
    )
    res.status(200).json({
      transaction,
      message: "Please approve the transaction to mint your NFT!",
    })
  } catch (error) {
    console.error(error)
    res.status(500).json({ error: "error creating transaction" })
    return
  }
}

async function buildTransaction(account: PublicKey, reference: PublicKey) {
  // Required solang dataAccount, even though we're not using it.
  const [dataAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("seed")],
    program.programId
  )

  // tree authority
  const [treeAuthority] = PublicKey.findProgramAddressSync(
    [treeAddress.toBuffer()],
    BUBBLEGUM_PROGRAM_ID
  )

  // Randomly select a uri.
  const randomUri = uris[Math.floor(Math.random() * uris.length)]

  // Initialize the dataAccount.
  const instruction = await program.methods
    .mint(
      randomUri // uri
    )
    .accounts({ 
        tree_authority: treeAuthority, // treeAuthority
        leaf_owner: account, // leafOwner
        leaf_delegate: account, // leafDelegate
        merkle_tree: account, // merkleTree
        payer: account, // payer
        tree_delegate: account, // treeDelegate, public tree (no delegate check, just require signer)
        noop_address: SPL_NOOP_PROGRAM_ID,
        compression_pid: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        bubblegum_pid: BUBBLEGUM_PROGRAM_ID,
    })
    .instruction()

  // Add the reference account to the instruction
  // Used in client to find the transaction once sent
  instruction.keys.push({
    pubkey: reference,
    isSigner: false,
    isWritable: false,
  })

  const latestBlockhash = await connection.getLatestBlockhash()

  // create new Transaction and add instruction
  const transaction = new Transaction({
    feePayer: account,
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  }).add(instruction)

  return transaction
    .serialize({ requireAllSignatures: false })
    .toString("base64")
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    return get(res)
  } else if (req.method === "POST") {
    return await post(req, res)
  } else {
    return res.status(405).json({ error: "Method not allowed" })
  }
}
