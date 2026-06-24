import { createSolanaRpc } from '@solana/kit';

export type CustomNetwork = 'mainnet' | 'devnet' | 'testnet';

const URL_KEY = 'custom-rpc-url';
const LABEL_KEY = 'custom-rpc-label';
const NETWORK_KEY = 'custom-rpc-network';
const SETUP_CLUSTER_KEY = 'setup-cluster';

const GENESIS_TO_NETWORK: Record<string, CustomNetwork> = {
    '5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d': 'mainnet',
    EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG: 'devnet',
    '4uhcVJyU9pJkvQyS88uRDiswHXSCkY3zQawwpjk2NsNY': 'testnet',
};

export interface CustomRpc {
    label: string;
    network: CustomNetwork;
    url: string;
}

export function readCustomRpc(): CustomRpc | null {
    const url = localStorage.getItem(URL_KEY);
    const network = localStorage.getItem(NETWORK_KEY) as CustomNetwork | null;
    if (!url || !network) return null;
    return { label: localStorage.getItem(LABEL_KEY) || 'Custom', network, url };
}

export async function detectNetwork(url: string): Promise<CustomNetwork | null> {
    const genesisHash = await createSolanaRpc(url).getGenesisHash().send();
    return GENESIS_TO_NETWORK[genesisHash] ?? null;
}

export function saveCustomRpc(url: string, network: CustomNetwork, label?: string): void {
    localStorage.setItem(URL_KEY, url);
    localStorage.setItem(NETWORK_KEY, network);
    localStorage.setItem(LABEL_KEY, label?.trim() || 'Custom');
    localStorage.setItem(SETUP_CLUSTER_KEY, `solana:${network}`);
    localStorage.setItem(`setup-complete-${network}`, 'true');
}

export function clearCustomRpc(): void {
    localStorage.removeItem(URL_KEY);
    localStorage.removeItem(NETWORK_KEY);
    localStorage.removeItem(LABEL_KEY);
}

export function isValidRpcUrl(value: string): boolean {
    try {
        const { protocol } = new URL(value);
        return protocol === 'http:' || protocol === 'https:';
    } catch {
        return false;
    }
}
