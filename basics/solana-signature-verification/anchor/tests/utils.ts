import {
    LAMPORTS_PER_SOL,
    SystemProgram,
    Transaction,
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction
} from "@solana/web3.js";
import { HermesClient } from "@pythnetwork/hermes-client";

export async function transferLamports(
    connection: Connection,
    fromKeypair: Keypair,
    toPubkey: PublicKey,
    amountInSol: number
) {
    try {

        // Create transfer transaction
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: fromKeypair.publicKey,
                toPubkey: toPubkey,
                lamports: amountInSol * LAMPORTS_PER_SOL,
            })
        );
        const { blockhash } = await connection.getLatestBlockhash("finalized");
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = fromKeypair.publicKey;
        // Sign and send transaction
        const signature = await sendAndConfirmTransaction(connection, transaction, [fromKeypair]);
        console.log(`✅ Transfer successful! TX Signature: ${signature}`);
        return signature;
    } catch (error) {
        console.error("❌ Transfer failed:", error);
        throw error;
    }
}
export async function getPriceUpdateData(feed_id: string) {
    const priceServiceConnection = new HermesClient(
        "https://hermes.pyth.network/",
        {}
    );

    const response = await priceServiceConnection.getLatestPriceUpdates(
        [feed_id],
        { encoding: "base64" }
    );

    return response.binary.data;
}