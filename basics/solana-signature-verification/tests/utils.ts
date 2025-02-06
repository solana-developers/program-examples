import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { AnchorWallet } from "@solana/wallet-adapter-react";
import {
    PythSolanaReceiver,
    InstructionWithEphemeralSigners,
} from "@pythnetwork/pyth-solana-receiver";
import { HermesClient } from "@pythnetwork/hermes-client";
import bs58 from "bs58";
import { Ed25519Program, SignatureResult, LAMPORTS_PER_SOL, Signer, VersionedTransaction, SystemProgram, Transaction, Connection, Keypair, PublicKey, sendAndConfirmTransaction } from "@solana/web3.js";
import { SolanaSignatureVerification } from "../target/types/solana_signature_verification";
import * as ed from '@noble/ed25519';
import * as fs from 'fs';
import * as path from 'path';

import { confirmTransaction } from "@solana-developers/helpers";
import { assert, expect } from 'chai';
const TX_RETRY_INTERVAL = 500
const isVersionedTransaction = (
    tx: Transaction | VersionedTransaction
): tx is VersionedTransaction => {
    return "version" in tx;
};
export async function sendTransactions(
    transactions: {
        tx: VersionedTransaction | Transaction;
        signers?: Signer[] | undefined;
    }[],
    connection: Connection,
    wallet: AnchorWallet,
    maxRetries?: number
): Promise<string[]> {
    const blockhashResult = await connection.getLatestBlockhashAndContext({
        commitment: "confirmed",
    });

    const signatures: string[] = [];

    // Signing logic for versioned transactions is different from legacy transactions
    for (const [index, transaction] of transactions.entries()) {
        const signers = transaction.signers;
        let tx = transaction.tx;

        if (isVersionedTransaction(tx)) {
            if (signers) {
                tx.sign(signers);
            }
        } else {
            tx.feePayer = tx.feePayer ?? wallet.publicKey;
            tx.recentBlockhash = blockhashResult.value.blockhash;

            if (signers) {
                for (const signer of signers) {
                    tx.partialSign(signer);
                }
            }
        }

        tx = await wallet.signTransaction(tx);

        // In the following section, we wait and constantly check for the transaction to be confirmed
        // and resend the transaction if it is not confirmed within a certain time interval
        // thus handling tx retries on the client side rather than relying on the RPC
        let confirmedTx: SignatureResult | null = null;
        let retryCount = 0;

        // Get the signature of the transaction with different logic for versioned transactions
        const txSignature = bs58.encode(
            isVersionedTransaction(tx)
                ? tx.signatures?.[0] || new Uint8Array()
                : tx.signature ?? new Uint8Array()
        );

        const confirmTransactionPromise = connection.confirmTransaction(
            {
                signature: txSignature,
                blockhash: blockhashResult.value.blockhash,
                lastValidBlockHeight: blockhashResult.value.lastValidBlockHeight,
            },
            "confirmed"
        );

        confirmedTx = null;
        while (!confirmedTx) {
            confirmedTx = await Promise.race([
                new Promise<SignatureResult>((resolve) => {
                    confirmTransactionPromise.then((result) => {
                        resolve(result.value);
                    });
                }),
                new Promise<null>((resolve) =>
                    setTimeout(() => {
                        resolve(null);
                    }, TX_RETRY_INTERVAL)
                ),
            ]);
            if (confirmedTx) {
                break;
            }
            if (maxRetries && maxRetries < retryCount) {
                break;
            }
            console.log(
                "Retrying transaction ",
                index,
                " of ",
                transactions.length - 1,
                " with signature: ",
                txSignature,
                " Retry count: ",
                retryCount
            );
            retryCount++;

            await connection.sendRawTransaction(tx.serialize(), {
                // Skipping preflight i.e. tx simulation by RPC as we simulated the tx above
                // This allows Triton RPCs to send the transaction through multiple pathways for the fastest delivery
                skipPreflight: true,
                // Setting max retries to 0 as we are handling retries manually
                // Set this manually so that the default is skipped
                maxRetries: 0,
                preflightCommitment: "confirmed",
            });
        }
        if (confirmedTx?.err) {
            throw new Error(
                `Transaction ${txSignature} has failed with error: ${JSON.stringify(
                    confirmedTx.err
                )}`
            );
        }

        if (!confirmedTx) {
            throw new Error("Failed to land the transaction");
        }

        signatures.push(txSignature);
    }

    return signatures;
}
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