/**
 * Initializes the World Cup config, oracle, and vault on-chain.
 *
 * Env:
 *   ADMIN   - admin keypair secret as a JSON [u8] array (64 bytes)
 *   RPC_URL - cluster endpoint (default: http://127.0.0.1:8899)
 */

import { createClient, createKeyPairSignerFromBytes } from '@solana/kit';
import { solanaRpc } from '@solana/kit-plugin-rpc';
import { signer } from '@solana/kit-plugin-signer';
import { getInitConfigInstructionAsync } from '@solana/world-cup';

const RPC_URL = process.env.RPC_URL ?? 'http://127.0.0.1:8899';

const REGISTRATION_WINDOW_SECONDS = 5 * 24 * 60 * 60;

function adminSecret(): Uint8Array {
    const raw = process.env.ADMIN;
    if (!raw) throw new Error('ADMIN env var is required (JSON [u8] array of the keypair secret)');
    return Uint8Array.from(JSON.parse(raw) as number[]);
}

function lockTs(): bigint {
    return BigInt(Math.floor(Date.now() / 1000) + REGISTRATION_WINDOW_SECONDS);
}

async function main() {
    const admin = await createKeyPairSignerFromBytes(adminSecret());
    const client = createClient()
        .use(signer(admin))
        .use(solanaRpc({ rpcUrl: RPC_URL }));

    console.log(`Admin:   ${admin.address}`);
    console.log(`RPC:     ${RPC_URL}`);

    const instruction = await getInitConfigInstructionAsync({
        admin,
        initConfigData: { lockTs: lockTs() },
    });

    const { context } = await client.sendTransaction([instruction]);
    console.log(`Config initialized: ${context.signature}`);
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
