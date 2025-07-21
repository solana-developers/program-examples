'use client'

import { getABLTokenProgram, getABLTokenProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey, Transaction } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../use-transaction-toast'
import { toast } from 'sonner'
import { BN } from '@coral-xyz/anchor'
import { amountToUiAmount, createAssociatedTokenAccountIdempotentInstruction, createAssociatedTokenAccountIdempotentInstructionWithDerivation, createMintToCheckedInstruction, decodeMintToCheckedInstruction, getAssociatedTokenAddressSync, getMint, getPermanentDelegate, getTokenMetadata, getTransferHook, mintToChecked, mintToCheckedInstructionData, TOKEN_2022_PROGRAM_ID } from '@solana/spl-token'


export function useHasTransferHookEnabled(mint: PublicKey) {
  const { connection } = useConnection()
  const provider = useAnchorProvider()
  const { cluster } = useCluster()
  const programId = useMemo(() => getABLTokenProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getABLTokenProgram(provider, programId), [provider, programId])
  return useQuery({
    queryKey: ['has-transfer-hook', { cluster }],
    queryFn: async () => {
      const mintInfo = await getMint(
        connection,
        mint,
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
        );
      const transferHook = getTransferHook(mintInfo);
      return transferHook !== null && programId.equals(transferHook.programId);
    },
  })
}
export function useAblTokenProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getABLTokenProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getABLTokenProgram(provider, programId), [provider, programId])

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initToken = useMutation({
    mutationKey: ['abl-token', 'init-token', { cluster }],
    mutationFn: (args: {
      mintAuthority: PublicKey,
      freezeAuthority: PublicKey,
      permanentDelegate: PublicKey,
      transferHookAuthority: PublicKey,
      mode: string,
      threshold: BN,
      name: string,
      symbol: string,
      uri: string,
      decimals: number,
    }) => {
      const modeEnum = args.mode === 'allow' ? { allow: {} } : args.mode === 'block' ? { block: {}} : { mixed: {}};
      const mint = Keypair.generate();

      return program.methods.initMint({
        decimals: args.decimals,
        mintAuthority: args.mintAuthority,
        freezeAuthority: args.freezeAuthority,
        permanentDelegate: args.permanentDelegate,
        transferHookAuthority: args.mintAuthority,
        mode: modeEnum,
        threshold: args.threshold,
        name: args.name,
        symbol: args.symbol,
        uri: args.uri,
      }).accounts({
        mint: mint.publicKey,
      }).signers([mint]).rpc().then((signature) => ({ signature, mintAddress: mint.publicKey }))
    },
    onSuccess: ({ signature, mintAddress }) => {
      transactionToast(signature)
      window.location.href = `/manage-token/${mintAddress.toString()}`
    },
    onError: () => toast.error('Failed to initialize token'),
  })

  const attachToExistingToken = useMutation({
    mutationKey: ['abl-token', 'attach-to-existing-token', { cluster }],
    mutationFn: (args: {
      mint: PublicKey,
      mode: string,
      threshold: BN,
      name: string | null,
      symbol: string | null,
      uri: string | null,
    }) => {
      const modeEnum = args.mode === 'allow' ? { allow: {} } : args.mode === 'block' ? { block: {}} : { mixed: {}};

      return program.methods.attachToMint({
        mode: modeEnum,
        threshold: args.threshold,
        name: args.name,
        symbol: args.symbol,
        uri: args.uri,
      }).accounts({
        mint: args.mint,
      }).rpc()
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to initialize token'),
  })

  const changeMode = useMutation({
    mutationKey: ['abl-token', 'change-mode', { cluster }],
    mutationFn: (args: {
      mode: string,
      threshold: BN,
      mint: PublicKey,
    }) => {
      const modeEnum = args.mode === 'allow' ? { allow: {} } : args.mode === 'block' ? { block: {}} : { mixed: {}}
      return program.methods.changeMode({
        mode: modeEnum,
        threshold: args.threshold,
      }).accounts({
        mint: args.mint,
      }).rpc()
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })

  const initWallet = useMutation({
    mutationKey: ['abl-token', 'change-mode', { cluster }],
    mutationFn: (args: {
      wallet: PublicKey,
      allowed: boolean,
    }) => {
      return program.methods.initWallet({
        allowed: args.allowed,
      }).accounts({
        wallet: args.wallet,
      }).rpc()
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })

  const processBatchWallets = useMutation({
    mutationKey: ['abl-token', 'process-batch-wallets', { cluster }],
    mutationFn: async (args: {
      wallets: {wallet: PublicKey, mode: "allow" | "block" | "remove"}[],
    }) => {
      const instructions = await Promise.all(args.wallets.map((wallet) => {
        if (wallet.mode === "remove") {
          const [abWalletPda] = PublicKey.findProgramAddressSync(
            [
              Buffer.from('ab_wallet'),
              wallet.wallet.toBuffer(),
            ],
            program.programId
          );
          return program.methods.removeWallet().accounts({
            abWallet: abWalletPda,
          }).instruction()
        }
        return program.methods.initWallet({
          allowed: wallet.mode === "allow",
        }).accounts({
          wallet: wallet.wallet,
        }).instruction()
      }));
      
      const transaction = new Transaction();
      transaction.add(...instructions);
      transaction.feePayer = provider.wallet.publicKey;
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      //transaction.sign(provider.wallet);

      let signedTx = await provider.wallet.signTransaction(transaction);

      return connection.sendRawTransaction(signedTx.serialize());

    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })
  

  const removeWallet = useMutation({
    mutationKey: ['abl-token', 'change-mode', { cluster }],
    mutationFn: (args: {
      wallet: PublicKey,
    }) => {
      const [abWalletPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('ab_wallet'),
          args.wallet.toBuffer(),
        ],
        program.programId
      );
      return program.methods.removeWallet().accounts({
        abWallet: abWalletPda,
      }).rpc()
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })


  const initConfig = useMutation({
    mutationKey: ['abl-token', 'init-config', { cluster }],
    mutationFn: () => {
      return program.methods.initConfig().rpc()
    },
  })

  const getConfig = useQuery({
    queryKey: ['get-config', { cluster }],
    queryFn: () => {
      const [configPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('config')],
        program.programId
      );
      return program.account.config.fetch(configPda)
    },
  })
  
  const getAbWallets = useQuery({
    queryKey: ['get-ab-wallets', { cluster }],
    queryFn: () => {
      return program.account.abWallet.all()
    },
  })

  const getToken = (mint: PublicKey) => useQuery({
    queryKey: ['get-token', { endpoint: connection.rpcEndpoint, mint }],
    queryFn: async () => {
      const mintInfo = await getMint(
        connection,
        mint,
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
      );

      const metadata = await getTokenMetadata(
        connection,
        mint,
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
      );

      const permanentDelegate = await getPermanentDelegate(mintInfo);

      return {
        name: metadata?.name,
        symbol: metadata?.symbol,
        uri: metadata?.uri,
        decimals: mintInfo.decimals,
        mintAuthority: mintInfo.mintAuthority,
        freezeAuthority: mintInfo.freezeAuthority,
        permanentDelegate: permanentDelegate,
      }
    },
  })
/*
  const getBalance = useQuery({
    queryKey: ['get-balance', { cluster }],
    queryFn: () => {
      getbal
    },
  })*/

  const mintTo = useMutation({
    mutationKey: ['abl-token', 'mint-to', { cluster }],
    mutationFn: async (args: {
      mint: PublicKey,
      amount: BN,
      recipient: PublicKey,
    }) => {
      const mintInfo = await getMint(
        connection,
        args.mint,
        "confirmed",
        TOKEN_2022_PROGRAM_ID,
      );
      const ata = getAssociatedTokenAddressSync(args.mint, args.recipient, true, TOKEN_2022_PROGRAM_ID);
      
      const ix = createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, ata, args.recipient, args.mint, TOKEN_2022_PROGRAM_ID);
      const ix2 = createMintToCheckedInstruction(args.mint, ata, provider.publicKey, args.amount.toNumber(), mintInfo.decimals, undefined, TOKEN_2022_PROGRAM_ID);
      const tx = new Transaction();
      tx.add(ix, ix2);
      tx.feePayer = provider.wallet.publicKey;
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      let signedTx = await provider.wallet.signTransaction(tx);
      return connection.sendRawTransaction(signedTx.serialize())
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })

  return {
    program,
    programId,
    getProgramAccount,
    initToken,
    changeMode,
    initWallet,
    removeWallet,
    initConfig,
    getConfig,
    getAbWallets,
    getToken,
    processBatchWallets,
    mintTo,
    attachToExistingToken,
  }
}
