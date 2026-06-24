import { getAddressFromPublicKey, createKeyPairFromPrivateKeyBytes } from '@solana/kit';
import crypto from 'node:crypto';
import { CHUNK_SIZE } from './bpf-loader.js';

export { CHUNK_SIZE };

/**
 * Generate a fresh ed25519 keypair and serialize it as the 64-byte
 * `[priv(32) | pub(32)]` Solana keypair format that `createKeyPairFromBytes`
 * expects on the client side. Replaces the removed kit helpers
 * `generateExtractableKeyPair` + `extractBytesFromKeyPair`.
 */
async function generateKeypairBytes(): Promise<{ keypairBytes: Uint8Array; publicKey: CryptoKey }> {
    const privBytes = crypto.getRandomValues(new Uint8Array(32));
    const kp = await createKeyPairFromPrivateKeyBytes(privBytes, true);
    const pubBytes = new Uint8Array(await crypto.subtle.exportKey('raw', kp.publicKey));
    const keypairBytes = new Uint8Array(64);
    keypairBytes.set(privBytes, 0);
    keypairBytes.set(pubBytes, 32);
    return { keypairBytes, publicKey: kp.publicKey };
}

export interface DeployPlan {
    bufferKeypair: number[];
    bufferAddress: string;
    chunks: string[];
    totalChunks: number;
    programAddress: string;
    soHash: string;
    soSize: number;
}

function chunkSoBytes(soBytes: Uint8Array): string[] {
    const totalChunks = Math.ceil(soBytes.length / CHUNK_SIZE);
    const chunks: string[] = [];
    for (let i = 0; i < totalChunks; i++) {
        const offset = i * CHUNK_SIZE;
        const chunk = soBytes.slice(offset, offset + CHUNK_SIZE);
        chunks.push(Buffer.from(chunk).toString('base64'));
    }
    return chunks;
}

export async function buildDeployPlan(soBytes: Uint8Array, programAddress: string): Promise<DeployPlan> {
    const soHash = crypto.createHash('sha256').update(soBytes).digest('hex');

    const { keypairBytes: bufferKeypairBytes, publicKey: bufferPubKey } = await generateKeypairBytes();
    const bufferAddress = await getAddressFromPublicKey(bufferPubKey);

    const chunks = chunkSoBytes(soBytes);

    return {
        bufferKeypair: Array.from(bufferKeypairBytes),
        bufferAddress: bufferAddress.toString(),
        chunks,
        totalChunks: chunks.length,
        programAddress,
        soHash,
        soSize: soBytes.length,
    };
}

export async function buildUpgradePlan(soBytes: Uint8Array, programAddress: string): Promise<DeployPlan> {
    const soHash = crypto.createHash('sha256').update(soBytes).digest('hex');

    const { keypairBytes: bufferKeypairBytes, publicKey: bufferPubKey } = await generateKeypairBytes();
    const bufferAddress = await getAddressFromPublicKey(bufferPubKey);

    const chunks = chunkSoBytes(soBytes);

    return {
        bufferKeypair: Array.from(bufferKeypairBytes),
        bufferAddress: bufferAddress.toString(),
        chunks,
        totalChunks: chunks.length,
        programAddress,
        soHash,
        soSize: soBytes.length,
    };
}
