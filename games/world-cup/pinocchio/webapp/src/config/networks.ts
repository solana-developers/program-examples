import type { Network } from '@/lib/cluster';

export interface TokenConfig {
    decimals: number;
    mint: string;
    name: string;
    symbol: string;
}

export interface NetworkConfig {
    programAddress: string | null;
    tokens: TokenConfig[];
}

const PROGRAM_ID = 'wCupoZtR1g1NXRRVELe5KqFgayyEteVKKxEerxugvxA';

const DEVNET_USDC = '4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU';
const MAINNET_USDC = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';

export const STATIC_NETWORKS: Record<Network, NetworkConfig> = {
    devnet: {
        programAddress: PROGRAM_ID,
        tokens: [{ decimals: 6, mint: DEVNET_USDC, name: 'USD Coin', symbol: 'USDC' }],
    },
    localnet: {
        programAddress: import.meta.env.VITE_LOCALNET_PROGRAM ?? PROGRAM_ID,
        tokens: import.meta.env.VITE_LOCALNET_USDC_MINT
            ? [
                  {
                      decimals: 6,
                      mint: import.meta.env.VITE_LOCALNET_USDC_MINT,
                      name: 'USD Coin (mock)',
                      symbol: 'USDC',
                  },
              ]
            : [],
    },
    mainnet: {
        programAddress: PROGRAM_ID,
        tokens: [{ decimals: 6, mint: MAINNET_USDC, name: 'USD Coin', symbol: 'USDC' }],
    },
    testnet: {
        programAddress: PROGRAM_ID,
        tokens: [],
    },
};
