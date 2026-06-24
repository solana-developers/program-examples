/**
 * Mints mock USDC to a recipient wallet
 * Usage: tsx scripts/mint-usdc.ts <recipient-address> <amount>
 */

import { address } from '@solana/kit';
import { getUsdcMint } from './config-manager.js';
import { mintMockUsdc } from './helpers.js';

async function main() {
    const args = process.argv.slice(2);

    if (args.length < 2) {
        console.error('Usage: tsx scripts/mint-usdc.ts <recipient-address> <amount>');
        console.error('Example: tsx scripts/mint-usdc.ts 7xyz... 1000');
        process.exit(1);
    }

    const [recipientAddress, amountStr] = args;
    const amount = parseFloat(amountStr);

    if (isNaN(amount) || amount <= 0) {
        console.error('Amount must be a positive number');
        process.exit(1);
    }

    try {
        const rpcUrl = process.env.RPC_URL ?? 'http://127.0.0.1:8899';
        const network = rpcUrl.includes('devnet')
            ? 'devnet'
            : rpcUrl.includes('testnet')
              ? 'testnet'
              : rpcUrl.includes('mainnet')
                ? 'mainnet'
                : 'localnet';
        const usdcMint = await getUsdcMint(network);
        if (!usdcMint) {
            console.error('USDC mint not configured. Run init-test-environment.ts first.');
            process.exit(1);
        }

        console.log(`Minting ${amount} USDC to ${recipientAddress}...`);

        // Convert to smallest units (6 decimals)
        const amountInSmallestUnits = BigInt(Math.round(amount * 1_000_000));

        await mintMockUsdc(address(usdcMint), address(recipientAddress), amountInSmallestUnits);

        console.log(`Successfully minted ${amount} USDC to ${recipientAddress}`);
    } catch (error) {
        console.error('Failed to mint USDC:', error);
        process.exit(1);
    }
}

main();
