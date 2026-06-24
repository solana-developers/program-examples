/**
 * Initializes test environment for webapp development
 * Idempotent - checks on-chain state before creating resources
 */

import { createSolanaRpc, address } from '@solana/kit';
import { readConfig, addToken, clearConfig, setProgramAddress } from './config-manager.js';
import { createMockUsdc } from './helpers.js';

const PROGRAM_ID = 'wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA';

const RPC_URL = process.env.RPC_URL ?? 'http://127.0.0.1:8899';
const NETWORK = process.env.NETWORK ?? 'localnet';

async function main() {
    console.log(`Initializing test environment for ${NETWORK}...`);

    try {
        const rpc = createSolanaRpc(RPC_URL);

        console.log(`Connecting to ${RPC_URL}...`);
        await rpc.getLatestBlockhash().send();
        await new Promise(r => setTimeout(r, 1000));
        console.log('RPC ready');

        const existingConfig = await readConfig();
        const networkTokens = existingConfig.networks[NETWORK]?.tokens ?? [];
        let usdcMintString = networkTokens.find(t => t.symbol === 'USDC')?.mint;
        let needsNewMint = true;

        if (usdcMintString) {
            try {
                const accountInfo = await rpc.getAccountInfo(address(usdcMintString)).send();
                if (accountInfo.value) {
                    console.log('Existing mock USDC mint found on-chain:', usdcMintString);
                    needsNewMint = false;
                }
            } catch {
                console.log('Configured USDC mint not found on-chain, will create new one');
            }
        }

        if (needsNewMint) {
            console.log('Creating new mock USDC mint...');
            usdcMintString = await createMockUsdc();
            console.log('New mock USDC created:', usdcMintString);
        }

        console.log('Updating configuration...');
        await clearConfig(NETWORK);
        console.log('Program ID:', PROGRAM_ID);
        await setProgramAddress(NETWORK, PROGRAM_ID);

        await addToken(NETWORK, {
            symbol: 'USDC',
            name: 'Mock USDC',
            mint: usdcMintString!,
            decimals: 6,
            type: 'test',
            description: 'Mock USDC for local development',
        });

        console.log('Test environment ready!');
        console.log('');
        console.log('USDC Mint:', usdcMintString);
    } catch (error) {
        console.error('Failed to initialize:', error);
        process.exit(1);
    }
}

main();
