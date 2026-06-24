import { useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { createSolanaRpc } from '@solana/kit';
import { useClusterConfig } from '@/hooks/use-cluster-config';

export function useClusterVersion() {
    const clusterConfig = useClusterConfig();
    const rpc = useMemo(() => createSolanaRpc(clusterConfig.url), [clusterConfig.url]);

    return useQuery({
        retry: 2,
        queryKey: ['version', { cluster: clusterConfig.id }],
        queryFn: () => rpc.getVersion().send(),
    });
}
