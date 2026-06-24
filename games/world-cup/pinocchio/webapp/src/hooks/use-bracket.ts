import { address } from '@solana/kit';
import { type Bracket, fetchMaybeBracketFromSeeds } from '@solana/world-cup';
import { useQuery } from '@tanstack/react-query';

import { useClusterConfig } from '@/hooks/use-cluster-config';
import { useRpc } from '@/hooks/use-rpc';

/** React-query key for one wallet's bracket on a given cluster. */
export function bracketQueryKey(clusterId: string, owner: string | null) {
    return ['bracket', clusterId, owner] as const;
}

/** Reads the Bracket PDA for an owner wallet; resolves to `null` when none exists. */
export function useBracket(owner: string | null) {
    const rpc = useRpc();
    const { id } = useClusterConfig();

    return useQuery<Bracket | null>({
        enabled: owner != null,
        queryFn: async () => {
            const account = await fetchMaybeBracketFromSeeds(rpc, { owner: address(owner!) });
            return account.exists ? account.data : null;
        },
        queryKey: bracketQueryKey(id, owner),
        staleTime: 15_000,
    });
}
