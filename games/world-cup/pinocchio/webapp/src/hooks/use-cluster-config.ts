import { useCluster } from '@solana/connector/react';

import type { ClusterWithUrl } from '@/lib/types';

export function useClusterConfig(): ClusterWithUrl {
    const { cluster } = useCluster();
    if (!cluster) return { id: 'solana:localnet', label: 'Localnet', url: '/rpc' };
    return { id: cluster.id, label: cluster.label, url: cluster.url };
}
