import { useQuery } from '@tanstack/react-query';

import { type NetworkConfig, STATIC_NETWORKS } from '@/config/networks';
import { useClusterConfig } from '@/hooks/use-cluster-config';
import { api } from '@/lib/api-client';
import { clusterIdToNetwork } from '@/lib/cluster';
import { readCustomTokens } from '@/lib/custom-tokens';

function withCustomTokens(network: string, config: NetworkConfig): NetworkConfig {
    const custom = readCustomTokens(network).filter(c => !config.tokens.some(t => t.mint === c.mint));
    if (custom.length === 0) return config;
    return { ...config, tokens: [...config.tokens, ...custom] };
}

export function useNetworkConfig() {
    const { id } = useClusterConfig();
    const network = clusterIdToNetwork(id);

    return useQuery<NetworkConfig>({
        queryFn: async () => {
            let base = STATIC_NETWORKS[network];
            if (import.meta.env.DEV) {
                try {
                    base = await api.config.getNetworkConfig(network);
                } catch {
                    base = STATIC_NETWORKS[network];
                }
            }
            return withCustomTokens(network, base);
        },
        queryKey: ['network-config', network, import.meta.env.DEV],
        retry: 2,
        staleTime: 30_000,
    });
}

export function useTokenConfig() {
    const { data, ...rest } = useNetworkConfig();
    return { data: data?.tokens, ...rest };
}

export function useProgramAddress(): string | null {
    const { data } = useNetworkConfig();
    return data?.programAddress ?? null;
}

export function useUsdcMintRaw() {
    const { data: tokens, isLoading } = useTokenConfig();
    return {
        isLoading,
        mint: tokens?.find(t => t.symbol === 'USDC')?.mint ?? null,
    };
}

export function useUsdcMint(): string | null {
    const { data: tokens } = useTokenConfig();
    return tokens?.find(t => t.symbol === 'USDC')?.mint ?? null;
}

export function useUsdcConfig() {
    const { data: tokens, ...rest } = useTokenConfig();
    const usdc = tokens?.find(t => t.symbol === 'USDC') ?? null;
    return { data: usdc, ...rest };
}
