export type Network = 'localnet' | 'devnet' | 'testnet' | 'mainnet';

export function clusterIdToNetwork(id: string): Network {
    if (id.includes('devnet')) return 'devnet';
    if (id.includes('testnet')) return 'testnet';
    if (id.includes('mainnet')) return 'mainnet';
    return 'localnet';
}
