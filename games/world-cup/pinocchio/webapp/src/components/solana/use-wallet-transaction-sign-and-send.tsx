import { useMemo } from 'react';
import type { Instruction, TransactionSigner } from '@solana/kit';
import {
    appendTransactionMessageInstructions,
    assertIsTransactionWithBlockhashLifetime,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    createTransactionMessage,
    estimateAndSetResourceLimitsFactory,
    estimateResourceLimitsFactory,
    getSignatureFromTransaction,
    pipe,
    sendAndConfirmTransactionFactory,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
} from '@solana/kit';
import { useClusterConfig } from '@/hooks/use-cluster-config';
import type { ClusterWithUrl } from '@/lib/types';

/**
 * Websocket endpoint for signature confirmation. Public clusters serve WS on the
 * same host as HTTP (`https://…` → `wss://…`); localnet's HTTP `/rpc` is proxied to
 * the validator, but subscriptions need its default WS port directly.
 */
function deriveWebsocketUrl(cluster: ClusterWithUrl): string {
    if (cluster.id === 'solana:localnet') return 'ws://localhost:8900';
    return cluster.url.replace(/^http/, 'ws');
}

export function useWalletTransactionSignAndSend() {
    const clusterConfig = useClusterConfig();
    const rpc = useMemo(() => createSolanaRpc(clusterConfig.url), [clusterConfig.url]);
    const sendAndConfirm = useMemo(() => {
        const rpcSubscriptions = createSolanaRpcSubscriptions(deriveWebsocketUrl(clusterConfig));
        return sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
    }, [rpc, clusterConfig]);
    const estimateAndSetResourceLimits = useMemo(() => {
        const estimate = estimateResourceLimitsFactory({ rpc });
        // Pad the simulated estimate: the bare value runs right at the limit, so real
        // execution (and the wallet's pre-sign simulation) can tip over it — fatal for
        // submit_bracket's self-CPI event emission. 10% + a small fixed floor, capped at max.
        const estimateWithHeadroom: typeof estimate = async (message, config) => {
            const limits = await estimate(message, config);
            return {
                ...limits,
                computeUnitLimit: Math.min(1_400_000, Math.ceil(limits.computeUnitLimit * 1.1) + 300),
            };
        };
        return estimateAndSetResourceLimitsFactory(estimateWithHeadroom);
    }, [rpc]);

    return async (ix: Instruction | Instruction[], signer: TransactionSigner): Promise<string> => {
        const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
        const instructions = Array.isArray(ix) ? ix : [ix];

        const transaction = pipe(
            createTransactionMessage({ version: 0 }),
            tx => setTransactionMessageFeePayerSigner(signer, tx),
            tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            tx => appendTransactionMessageInstructions(instructions, tx),
        );

        // Simulate to size the compute-unit limit (and surface program errors pre-flight).
        const transactionWithLimits = await estimateAndSetResourceLimits(transaction);

        const signedTransaction = await signTransactionMessageWithSigners(transactionWithLimits);
        assertIsTransactionWithBlockhashLifetime(signedTransaction);
        await sendAndConfirm(signedTransaction, { commitment: 'confirmed' });
        return getSignatureFromTransaction(signedTransaction);
    };
}
