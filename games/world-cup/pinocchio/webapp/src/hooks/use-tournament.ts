import {
    type Config,
    fetchMaybeConfigFromSeeds,
    fetchMaybeOracleFromSeeds,
    type Oracle,
    TournamentState,
} from '@solana/world-cup';
import { useQuery } from '@tanstack/react-query';

import { useClusterConfig } from '@/hooks/use-cluster-config';
import { useRpc } from '@/hooks/use-rpc';

/** Tournament-wide state read from the Config + Oracle singletons. */
export interface TournamentInfo {
    config: Config | null;
    /** Whether the oracle has posted at least one game result. */
    hasResults: boolean;
    /** Whether the program has been initialized (Config exists) on this cluster. */
    initialized: boolean;
    oracle: Oracle | null;
    /** Whether a new bracket can still be submitted (registration open, before lock). */
    registrationOpen: boolean;
}

/** Reads the Config and Oracle singletons and derives the tournament's current phase. */
export function useTournament() {
    const rpc = useRpc();
    const { id } = useClusterConfig();

    return useQuery<TournamentInfo>({
        queryFn: async () => {
            const [configAccount, oracleAccount] = await Promise.all([
                fetchMaybeConfigFromSeeds(rpc),
                fetchMaybeOracleFromSeeds(rpc),
            ]);
            const config = configAccount.exists ? configAccount.data : null;
            const oracle = oracleAccount.exists ? oracleAccount.data : null;
            const now = BigInt(Math.floor(Date.now() / 1000));
            const registrationOpen =
                config != null && config.state === TournamentState.Registration && now < config.lockTs;
            const hasResults = oracle != null && oracle.decidedMask !== 0;
            return { config, hasResults, initialized: config != null, oracle, registrationOpen };
        },
        queryKey: ['tournament', id],
        staleTime: 30_000,
    });
}
