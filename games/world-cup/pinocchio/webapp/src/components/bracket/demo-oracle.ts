import { GAME_COUNT, type Oracle, UNDECIDED } from '@solana/world-cup';

import { competitorsOf, type Pick } from './rounds';

/**
 * TEMPORARY dev helper: a synthetic Oracle so a submitted bracket can be
 * previewed with wins/losses/points before any real results are posted.
 *
 * Simulates a tournament partway through — every Round-of-32 and Round-of-16
 * game is decided (~75% of them matching the bracket's own pick) and later
 * rounds are left undecided. Deterministic per game so the preview is stable
 * across renders. Remove this file and the dev toggle in `bracket-display.tsx`
 * once real oracle results are available.
 */
export function demoOracleFromPicks(picks: number[]): Oracle {
    const results: number[] = Array.from({ length: GAME_COUNT }, () => UNDECIDED);
    let decidedMask = 0;

    for (let game = 0; game <= 23; game++) {
        const [a, b] = competitorsOf(game, picks);
        if (a == null || b == null) continue;
        const pick = picks[game];
        const other = pick === a ? b : a;
        const hit = (game * 7 + 3) % 4 !== 0;
        results[game] = hit ? pick : other;
        decidedMask |= 1 << game;
    }

    return {
        bump: 0,
        decidedMask,
        discriminator: 1,
        goalsPosted: 1,
        results,
        totalGoalsR32: 81,
    };
}

/**
 * TEMPORARY dev helper: a single synthetic Oracle for the whole field, so the
 * leaderboard can show a spread of scores before real results are posted. Unlike
 * {@link demoOracleFromPicks} (tailored to one bracket), this is one fixed "truth"
 * every bracket is scored against — a fully-decided tournament built by walking a
 * deterministic winner through the bracket tree. Brackets closer to that truth rank
 * higher. Remove alongside the dev toggle in `leaderboard.tsx`.
 */
export function demoLeaderboardOracle(): Oracle {
    const truth: Pick[] = Array.from({ length: GAME_COUNT }, () => null);
    for (let game = 0; game < GAME_COUNT; game++) {
        const [a, b] = competitorsOf(game, truth);
        if (a == null) truth[game] = b ?? 0;
        else if (b == null) truth[game] = a;
        // A deterministic mix so the truth isn't pure chalk: mostly `a`, sometimes `b`.
        else truth[game] = (game * 7 + 3) % 3 === 0 ? b : a;
    }

    return {
        bump: 0,
        decidedMask: 0xffffffff,
        discriminator: 1,
        goalsPosted: 1,
        results: truth.map(t => t ?? UNDECIDED),
        totalGoalsR32: 81,
    };
}
