import { FINAL_GAME, GAME_COUNT, randomBracket, type SubmitBracketDataArgs, THIRD_PLACE_GAME } from '@solana/world-cup';
import { useCallback, useMemo, useState } from 'react';

import { competitorsOf, type Pick } from './rounds';

/** Largest tiebreaker the program accepts (a `u16`). */
export const TIEBREAKER_MAX = 65535;

function emptyPicks(): Pick[] {
    return Array.from({ length: GAME_COUNT }, () => null);
}

/** Live bracket state plus the actions the builder UI drives it with. */
export interface BracketBuilder {
    champion: Pick;
    finalists: [Pick, Pick];
    isComplete: boolean;
    pickCount: number;
    picks: Pick[];
    randomize: () => SubmitBracketDataArgs;
    reset: () => void;
    setBracket: (data: SubmitBracketDataArgs) => void;
    setTiebreaker: (value: number | null) => void;
    setWinner: (game: number, slot: number) => void;
    thirdPlace: Pick;
    tiebreaker: number | null;
    toSubmitData: () => SubmitBracketDataArgs | null;
}

export function useBracketBuilder(): BracketBuilder {
    const [picks, setPicks] = useState<Pick[]>(emptyPicks);
    const [tiebreaker, setTiebreakerState] = useState<number | null>(null);

    const setWinner = useCallback((game: number, slot: number) => {
        setPicks(prev => {
            const next = prev.slice();
            next[game] = slot;
            for (let g = 16; g < GAME_COUNT; g++) {
                const advanced = next[g];
                if (advanced == null) continue;
                const [a, b] = competitorsOf(g, next);
                if (advanced !== a && advanced !== b) next[g] = null;
            }
            return next;
        });
    }, []);

    const setTiebreaker = useCallback((value: number | null) => {
        if (value == null) {
            setTiebreakerState(null);
            return;
        }
        setTiebreakerState(Math.max(0, Math.min(TIEBREAKER_MAX, Math.floor(value))));
    }, []);

    const reset = useCallback(() => {
        setPicks(emptyPicks());
        setTiebreakerState(null);
    }, []);

    const setBracket = useCallback((data: SubmitBracketDataArgs) => {
        setPicks(data.picks.slice());
        setTiebreakerState(data.tiebreakerGuess);
    }, []);

    const randomize = useCallback((): SubmitBracketDataArgs => {
        const data = randomBracket();
        setBracket(data);
        return data;
    }, [setBracket]);

    const pickCount = useMemo(() => picks.reduce<number>((n, p) => (p != null ? n + 1 : n), 0), [picks]);
    const finalists = useMemo(() => competitorsOf(FINAL_GAME, picks), [picks]);
    const isComplete = pickCount === GAME_COUNT && tiebreaker != null;

    const toSubmitData = useCallback((): SubmitBracketDataArgs | null => {
        if (pickCount !== GAME_COUNT || tiebreaker == null) return null;
        return { picks: picks.map(p => p as number), tiebreakerGuess: tiebreaker };
    }, [picks, pickCount, tiebreaker]);

    return {
        champion: picks[FINAL_GAME],
        finalists,
        isComplete,
        pickCount,
        picks,
        randomize,
        reset,
        setBracket,
        setTiebreaker,
        setWinner,
        thirdPlace: picks[THIRD_PLACE_GAME],
        tiebreaker,
        toSubmitData,
    };
}
