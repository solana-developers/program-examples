import { createSolanaRpc } from '@solana/kit';
import { useMemo } from 'react';

import { useClusterConfig } from '@/hooks/use-cluster-config';

export function useRpc() {
    const { url } = useClusterConfig();
    return useMemo(() => createSolanaRpc(url), [url]);
}
