import { spawn } from 'child_process';
import { createSolanaRpc, type Address } from '@solana/kit';

const RPC_URL = process.env.RPC_URL ?? 'http://127.0.0.1:8899';

export interface SolanaClient {
    rpc: ReturnType<typeof createSolanaRpc>;
}

export async function createSolanaClient(rpcUrl: string): Promise<SolanaClient> {
    const rpc = createSolanaRpc(rpcUrl);
    return { rpc };
}

function runCommand(command: string, args: string[]): Promise<string> {
    return new Promise((resolve, reject) => {
        const child = spawn(command, args);
        let stdout = '';
        let stderr = '';

        child.stdout.on('data', data => (stdout += data.toString()));
        child.stderr.on('data', data => (stderr += data.toString()));

        child.on('close', code => {
            if (code !== 0) {
                reject(new Error(`Command failed: ${stderr || stdout}`));
            } else {
                resolve(stdout);
            }
        });

        child.on('error', err => reject(err));
    });
}

/**
 * Creates a mock USDC token mint using spl-token CLI (Token-2022 program)
 */
export async function createMockUsdc(): Promise<Address<string>> {
    try {
        // Create a new token with 6 decimals using Token-2022 program
        const output = await runCommand('spl-token', [
            'create-token',
            '--decimals',
            '6',
            '--program-id',
            'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb', // Token-2022
            '--url',
            RPC_URL,
            '--fee-payer',
            `${process.env.HOME}/.config/solana/id.json`,
        ]);

        // Parse the mint address from output
        // Output format: "Creating token <ADDRESS>\nSignature: <SIG>"
        const match = output.match(/Creating token ([A-Za-z0-9]+)/);
        if (!match) {
            throw new Error(`Could not parse mint address from output: ${output}`);
        }

        const mintAddress = match[1] as Address<string>;
        console.log('Created mock USDC mint:', mintAddress);
        return mintAddress;
    } catch (e) {
        console.error('Failed to create mock USDC token');
        console.error(e);
        throw e;
    }
}

/**
 * Mints mock USDC tokens to a recipient using spl-token CLI
 */
export async function mintMockUsdc(mint: Address<string>, recipient: Address<string>, amount: bigint): Promise<void> {
    try {
        // First, try to create the token account for the recipient
        try {
            await runCommand('spl-token', [
                'create-account',
                mint,
                '--owner',
                recipient,
                '--url',
                RPC_URL,
                '--fee-payer',
                `${process.env.HOME}/.config/solana/id.json`,
            ]);
            console.log('Created token account for recipient');
        } catch {
            // Account might already exist, that's ok
            console.log('Token account already exists or creation skipped');
        }

        // Convert from smallest units to UI amount (6 decimals)
        const uiAmount = Number(amount) / 1_000_000;

        // Mint tokens to the recipient
        await runCommand('spl-token', [
            'mint',
            mint,
            String(uiAmount),
            '--recipient-owner',
            recipient,
            '--url',
            RPC_URL,
            '--fee-payer',
            `${process.env.HOME}/.config/solana/id.json`,
        ]);

        console.log(`Minted ${uiAmount} USDC to ${recipient}`);
    } catch (e) {
        console.error('Failed to mint mock USDC tokens');
        console.error('Mint address:', mint);
        console.error('Recipient:', recipient);
        console.error('Amount:', amount);
        console.error(e);
        throw e;
    }
}
