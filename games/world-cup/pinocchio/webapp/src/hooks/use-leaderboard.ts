import { type BracketAccount, fetchAllBrackets, fetchMaybeOracleFromSeeds, type Oracle } from '@solana/world-cup';
import { useQuery } from '@tanstack/react-query';

import { useClusterConfig } from '@/hooks/use-cluster-config';
import { useRpc } from '@/hooks/use-rpc';

/** Every bracket plus the oracle they should be scored against; ranking is derived by the caller. */
export interface LeaderboardData {
    brackets: BracketAccount[];
    /** Whether the oracle has decided at least one game (live scores are meaningful). */
    hasResults: boolean;
    oracle: Oracle | null;
}

/**
 * Fetches every bracket via `getProgramAccounts` and the oracle singleton. One
 * program-accounts call covers all entrants — each bracket carries its own picks, so
 * there's no per-wallet follow-up fetch. Ranking (via `buildLeaderboard`) is left to
 * the caller so a preview oracle can be swapped in.
 */
export function useLeaderboard() {
    const rpc = useRpc();
    const { id } = useClusterConfig();

    return useQuery<LeaderboardData>({
        queryFn: async () => {
            const [brackets, oracleAccount] = await Promise.all([
                fetchAllBrackets(rpc),
                fetchMaybeOracleFromSeeds(rpc),
            ]);
            const oracle = oracleAccount.exists ? oracleAccount.data : null;
            return {
                brackets,
                hasResults: oracle != null && oracle.decidedMask !== 0,
                oracle,
            };
        },
        queryKey: ['leaderboard', id],
        // No interval polling: results post at most ~1x/2h, so it isn't worth re-scanning
        // getProgramAccounts on a timer. react-query refetches on mount and on window focus,
        // so the board is fresh whenever it's opened or refocused.
        staleTime: 60_000,
    });
}
