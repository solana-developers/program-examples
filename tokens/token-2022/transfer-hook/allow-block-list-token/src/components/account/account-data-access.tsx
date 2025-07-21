'use client'

import { createAssociatedTokenAccountIdempotentInstruction, createTransferCheckedInstruction, createTransferCheckedWithTransferHookInstruction, getAssociatedTokenAddressSync, getExtraAccountMetaAddress, getExtraAccountMetas, getMint, getTransferHook, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  SendTransactionError,
  SystemProgram,
  Transaction,
  TransactionMessage,
  TransactionSignature,
  VersionedTransaction,
} from '@solana/web3.js'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useTransactionErrorToast, useTransactionToast } from '../use-transaction-toast'
import { useAnchorProvider } from '../solana/solana-provider'
import { toast } from 'sonner'
import { Buffer } from "buffer"

export function useGetBalance({ address }: { address: PublicKey }) {
  const { connection } = useConnection()

  return useQuery({
    queryKey: ['get-balance', { endpoint: connection.rpcEndpoint, address }],
    queryFn: () => connection.getBalance(address),
  })
}

export function useGetSignatures({ address }: { address: PublicKey }) {
  const { connection } = useConnection()

  return useQuery({
    queryKey: ['get-signatures', { endpoint: connection.rpcEndpoint, address }],
    queryFn: () => connection.getSignaturesForAddress(address),
  })
}

export function useSendTokens() {
  const { connection } = useConnection()
  const { publicKey } = useWallet()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const transactionErrorToast = useTransactionErrorToast()

  return useMutation({
    mutationFn: async (args: {
      mint: PublicKey,
      destination: PublicKey,
      amount: number,
    }) => {
      if (!publicKey) throw new Error('No public key found');
      const { mint, destination, amount } = args;
      const mintInfo = await getMint(connection, mint, 'confirmed', TOKEN_2022_PROGRAM_ID);
      const ataDestination = getAssociatedTokenAddressSync(mint, destination, true, TOKEN_2022_PROGRAM_ID);
      const ataSource = getAssociatedTokenAddressSync(mint, publicKey, true, TOKEN_2022_PROGRAM_ID);
      const ix = createAssociatedTokenAccountIdempotentInstruction(publicKey, ataDestination, destination, mint, TOKEN_2022_PROGRAM_ID);
      const bi = BigInt(amount);
      const decimals = mintInfo.decimals;
      console.log("BI: ", bi);
      console.log("AMOUNT: ", amount);
      console.log("DECIMALS: ", decimals);
      const buf = Buffer.alloc(10);
      console.dir(buf);
  
      buf.writeBigUInt64LE(bi, 0);
      console.log(buf);
      const ix3 = await createTransferCheckedInstruction(ataSource, mint, ataDestination, publicKey, bi, decimals, undefined, TOKEN_2022_PROGRAM_ID);

      const transferHook = getTransferHook(mintInfo);
      if (!transferHook) throw new Error('bad token');
      const extraMetas = getExtraAccountMetaAddress(mint, transferHook.programId);

      const seeds = [Buffer.from('ab_wallet'), destination.toBuffer()];
      const abWallet = PublicKey.findProgramAddressSync(seeds, transferHook.programId)[0];

      ix3.keys.push({ pubkey: abWallet, isSigner: false, isWritable: false });
      ix3.keys.push({ pubkey: transferHook.programId, isSigner: false, isWritable: false });
      ix3.keys.push({ pubkey: extraMetas, isSigner: false, isWritable: false });

      console.log("tx-hook: ", transferHook.programId.toString());
      console.log("extra-metas: ", extraMetas.toString());
      console.log("ab-wallet: ", abWallet.toString());
      console.log("KEYS: ", ix3.keys);
      
      const validateStateAccount = await connection.getAccountInfo(extraMetas, 'confirmed');
      if (!validateStateAccount) throw new Error('validate-state-account not found');
      const validateStateData = getExtraAccountMetas(validateStateAccount);
      console.log("validate-state-data: ", validateStateData);
    
      //const ix2 = await createTransferCheckedWithTransferHookInstruction(connection, ataSource, mint, ataDestination, publicKey, bi, decimals, undefined, 'confirmed', TOKEN_2022_PROGRAM_ID);
      
      const transaction = new Transaction();
      transaction.add(ix, ix3);
      transaction.feePayer = provider.wallet.publicKey;
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

      let signedTx = await provider.wallet.signTransaction(transaction);

      return connection.sendRawTransaction(signedTx.serialize());
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: (error) => { transactionErrorToast(error, connection) },
  })
}

export function useGetTokenAccounts({ address }: { address: PublicKey }) {
  const { connection } = useConnection()

  return useQuery({
    queryKey: ['get-token-accounts', { endpoint: connection.rpcEndpoint, address }],
    queryFn: async () => {
      const [tokenAccounts, token2022Accounts] = await Promise.all([
        connection.getParsedTokenAccountsByOwner(address, {
          programId: TOKEN_PROGRAM_ID,
        }),
        connection.getParsedTokenAccountsByOwner(address, {
          programId: TOKEN_2022_PROGRAM_ID,
        }),
      ])
      return [...tokenAccounts.value, ...token2022Accounts.value]
    },
  })
}

export function useTransferSol({ address }: { address: PublicKey }) {
  const { connection } = useConnection()
  // const transactionToast = useTransactionToast()
  const wallet = useWallet()
  const client = useQueryClient()

  return useMutation({
    mutationKey: ['transfer-sol', { endpoint: connection.rpcEndpoint, address }],
    mutationFn: async (input: { destination: PublicKey; amount: number }) => {
      let signature: TransactionSignature = ''
      try {
        const { transaction, latestBlockhash } = await createTransaction({
          publicKey: address,
          destination: input.destination,
          amount: input.amount,
          connection,
        })

        // Send transaction and await for signature
        signature = await wallet.sendTransaction(transaction, connection)

        // Send transaction and await for signature
        await connection.confirmTransaction({ signature, ...latestBlockhash }, 'confirmed')

        console.log(signature)
        return signature
      } catch (error: unknown) {
        console.log('error', `Transaction failed! ${error}`, signature)

        return
      }
    },
    onSuccess: (signature) => {
      if (signature) {
        // TODO: Add back Toast
        // transactionToast(signature)
        console.log('Transaction sent', signature)
      }
      return Promise.all([
        client.invalidateQueries({
          queryKey: ['get-balance', { endpoint: connection.rpcEndpoint, address }],
        }),
        client.invalidateQueries({
          queryKey: ['get-signatures', { endpoint: connection.rpcEndpoint, address }],
        }),
      ])
    },
    onError: (error) => {
      // TODO: Add Toast
      console.error(`Transaction failed! ${error}`)
    },
  })
}

export function useRequestAirdrop({ address }: { address: PublicKey }) {
  const { connection } = useConnection()
  // const transactionToast = useTransactionToast()
  const client = useQueryClient()

  return useMutation({
    mutationKey: ['airdrop', { endpoint: connection.rpcEndpoint, address }],
    mutationFn: async (amount: number = 1) => {
      const [latestBlockhash, signature] = await Promise.all([
        connection.getLatestBlockhash(),
        connection.requestAirdrop(address, amount * LAMPORTS_PER_SOL),
      ])

      await connection.confirmTransaction({ signature, ...latestBlockhash }, 'confirmed')
      return signature
    },
    onSuccess: (signature) => {
      // TODO: Add back Toast
      // transactionToast(signature)
      console.log('Airdrop sent', signature)
      return Promise.all([
        client.invalidateQueries({
          queryKey: ['get-balance', { endpoint: connection.rpcEndpoint, address }],
        }),
        client.invalidateQueries({
          queryKey: ['get-signatures', { endpoint: connection.rpcEndpoint, address }],
        }),
      ])
    },
  })
}

async function createTransaction({
  publicKey,
  destination,
  amount,
  connection,
}: {
  publicKey: PublicKey
  destination: PublicKey
  amount: number
  connection: Connection
}): Promise<{
  transaction: VersionedTransaction
  latestBlockhash: { blockhash: string; lastValidBlockHeight: number }
}> {
  // Get the latest blockhash to use in our transaction
  const latestBlockhash = await connection.getLatestBlockhash()

  // Create instructions to send, in this case a simple transfer
  const instructions = [
    SystemProgram.transfer({
      fromPubkey: publicKey,
      toPubkey: destination,
      lamports: amount * LAMPORTS_PER_SOL,
    }),
  ]

  // Create a new TransactionMessage with version and compile it to legacy
  const messageLegacy = new TransactionMessage({
    payerKey: publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions,
  }).compileToLegacyMessage()

  // Create a new VersionedTransaction which supports legacy and v0
  const transaction = new VersionedTransaction(messageLegacy)

  return {
    transaction,
    latestBlockhash,
  }
}
