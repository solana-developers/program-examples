import { useKitTransactionSigner } from '@solana/connector/react';
import {
    type Address,
    appendTransactionMessageInstructions,
    createSignerFromKeyPair,
    createTransactionMessage,
    generateKeyPair,
    getBase64EncodedWireTransaction,
    pipe,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
} from '@solana/kit';
import { findAssociatedTokenPda, getCreateAssociatedTokenIdempotentInstruction } from '@solana-program/token';
import {
    getInitializeMint2Instruction,
    getMintSize,
    getMintToInstruction,
    TOKEN_2022_PROGRAM_ADDRESS,
} from '@solana-program/token-2022';
import { useMutation } from '@tanstack/react-query';

import { useTransactionToast } from '@/components/use-transaction-toast';
import { useRpc } from '@/hooks/use-rpc';
import { buildCreateAccountIx, SYSTEM_PROGRAM } from '@/lib/bpf-loader-browser';

export function useCreateToken() {
    const { signer: walletSigner } = useKitTransactionSigner();
    const toast = useTransactionToast();
    const rpc = useRpc();

    async function signAndSendTransaction(tx: Parameters<typeof signTransactionMessageWithSigners>[0]) {
        const signedTx = await signTransactionMessageWithSigners(tx);
        await rpc.sendTransaction(getBase64EncodedWireTransaction(signedTx), { encoding: 'base64' }).send();
    }

    const createToken = useMutation({
        mutationFn: async ({ decimals = 6 }: { decimals?: number } = {}) => {
            if (!walletSigner) throw new Error('Wallet not connected');
            const signer = walletSigner;

            const mintKp = await createSignerFromKeyPair(await generateKeyPair());
            const mintSize = getMintSize();
            const rentLamports = await rpc.getMinimumBalanceForRentExemption(BigInt(mintSize)).send();
            const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

            const createAccIx = buildCreateAccountIx(
                signer,
                mintKp,
                rentLamports,
                mintSize,
                TOKEN_2022_PROGRAM_ADDRESS,
            );

            const initMintIx = getInitializeMint2Instruction({
                decimals,
                freezeAuthority: signer.address,
                mint: mintKp.address,
                mintAuthority: signer.address,
            });

            const tx = pipe(
                createTransactionMessage({ version: 0 }),
                m => setTransactionMessageFeePayerSigner(signer, m),
                m => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, m),
                m => appendTransactionMessageInstructions([createAccIx, initMintIx], m),
            );

            await signAndSendTransaction(tx);

            return { mint: mintKp.address };
        },
        onError: e => toast.onError(e),
        onSuccess: () => {
            toast.onSuccess('Token mint created');
        },
    });

    const mintTo = useMutation({
        mutationFn: async ({ mint, amount, recipient }: { amount: bigint; mint: Address; recipient?: Address }) => {
            if (!walletSigner) throw new Error('Wallet not connected');
            const signer = walletSigner;
            const owner = recipient ?? signer.address;

            const [ata] = await findAssociatedTokenPda({
                mint,
                owner,
                tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
            });

            const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

            const createAtaIx = getCreateAssociatedTokenIdempotentInstruction({
                ata,
                mint,
                owner,
                payer: signer,
                systemProgram: SYSTEM_PROGRAM,
                tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
            });

            const mintToIx = getMintToInstruction({
                amount,
                mint,
                mintAuthority: signer,
                token: ata,
            });

            const tx = pipe(
                createTransactionMessage({ version: 0 }),
                m => setTransactionMessageFeePayerSigner(signer, m),
                m => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, m),
                m => appendTransactionMessageInstructions([createAtaIx, mintToIx], m),
            );

            await signAndSendTransaction(tx);
            return { ata };
        },
        onError: e => toast.onError(e),
        onSuccess: () => {
            toast.onSuccess('Tokens minted');
        },
    });

    return { createToken, mintTo };
}
