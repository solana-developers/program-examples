import { useCluster, useWallet } from '@solana/connector/react';
import type { Address } from '@solana/kit';
import { createSolanaRpc } from '@solana/kit';
import { TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';
import { TOKEN_2022_PROGRAM_ADDRESS } from '@solana-program/token-2022';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';

import { useClusterConfig } from '@/hooks/use-cluster-config';
import { useCreateToken } from '@/hooks/use-create-token';
import { useRpc } from '@/hooks/use-rpc';
import { useUsdcMint } from '@/hooks/use-token-config';
import { api } from '@/lib/api-client';
import { invalidateWithDelay } from '@/lib/utils';

export function useGetBalanceQuery({ address: addr }: { address: Address }) {
    const clusterConfig = useClusterConfig();
    const rpc = useRpc();

    return useQuery({
        queryFn: () => rpc.getBalance(addr).send(),
        queryKey: ['get-balance', { address: addr, cluster: clusterConfig.id }],
        retry: false,
        staleTime: 5000, // Consider fresh for 5 seconds
    });
}

async function getTokenAccountsByOwner(
    rpc: ReturnType<typeof createSolanaRpc>,
    { address: addr, programId }: { address: Address; programId: Address },
) {
    const result = await rpc
        .getTokenAccountsByOwner(addr, { programId }, { commitment: 'confirmed', encoding: 'jsonParsed' })
        .send()
        .then(res => res.value ?? []);
    return result;
}

export function useGetTokenAccountsQuery({ address: addr }: { address: Address }) {
    const clusterConfig = useClusterConfig();
    const rpc = useRpc();

    return useQuery({
        queryFn: async () => {
            const result = await Promise.all([
                getTokenAccountsByOwner(rpc, { address: addr, programId: TOKEN_PROGRAM_ADDRESS }),
                getTokenAccountsByOwner(rpc, { address: addr, programId: TOKEN_2022_PROGRAM_ADDRESS }),
            ]).then(([tokenAccounts, token2022Accounts]) => [...tokenAccounts, ...token2022Accounts]);
            return result;
        },
        queryKey: ['get-token-accounts', { address: addr, cluster: clusterConfig.id }],
        // Consider fresh for 5 seconds
        refetchOnWindowFocus: true,
        staleTime: 5000,
    });
}

const LAMPORTS_PER_SOL = 1_000_000_000;

export function useRequestAirdropMutation({ address: addr }: { address: Address }) {
    const clusterConfig = useClusterConfig();
    const rpc = useRpc();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (amount: number = 1) => {
            const lamportAmount = BigInt(Math.round(amount * LAMPORTS_PER_SOL));
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const sig = await rpc.requestAirdrop(addr, lamportAmount as any).send();
            return sig;
        },
        onSuccess: async () => {
            await queryClient.invalidateQueries({
                queryKey: ['get-balance', { address: addr, cluster: clusterConfig.id }],
            });
        },
    });
}

export function useAirdropSol() {
    const { account } = useWallet();
    const { cluster } = useCluster();
    const rpc = useRpc();
    const queryClient = useQueryClient();
    const isDevnet = cluster?.id === 'solana:devnet';

    return useMutation({
        mutationFn: async (amount: number) => {
            if (!account) throw new Error('Wallet not connected');
            if (isDevnet) {
                const lamports = BigInt(Math.round(amount * LAMPORTS_PER_SOL));
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                await rpc.requestAirdrop(account, lamports as any).send();
                return { message: `Airdropped ${amount} SOL on devnet` };
            }
            return await api.airdrop.sol({ amount, recipient: account });
        },
        onError: error => {
            toast.error(`SOL airdrop failed: ${error.message}`);
        },
        onSuccess: result => {
            toast.success(result.message ?? 'SOL airdrop successful!');
            invalidateWithDelay(queryClient, [['get-balance']]);
        },
    });
}

export function useAirdropUsdc() {
    const { account } = useWallet();
    const { cluster } = useCluster();
    const queryClient = useQueryClient();
    const usdcMint = useUsdcMint();
    const { mintTo } = useCreateToken();
    const isDevnet = cluster?.id === 'solana:devnet';

    return useMutation({
        mutationFn: async ({ amount, recipient }: { amount: number; recipient?: string }) => {
            if (!account) throw new Error('Wallet not connected');
            if (isDevnet) {
                if (!usdcMint) throw new Error('USDC mint not configured');
                const rawAmount = BigInt(Math.round(amount * 1_000_000));
                const target = (recipient || account) as Address;
                await mintTo.mutateAsync({
                    amount: rawAmount,
                    mint: usdcMint as Address,
                    recipient: target,
                });
                return { message: `Minted ${amount} USDC to ${target.slice(0, 8)}...` };
            }
            return await api.airdrop.usdc({ amount, recipient: account });
        },
        onError: error => {
            toast.error(`USDC airdrop failed: ${error.message}`);
        },
        onSuccess: result => {
            toast.success(result.message ?? 'USDC airdrop successful!');
            invalidateWithDelay(queryClient, [['get-token-accounts']]);
        },
    });
}
