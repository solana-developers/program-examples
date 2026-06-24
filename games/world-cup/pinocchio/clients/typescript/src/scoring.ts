import { GAME_COUNT, roundOf } from './bracket.js';

/**
 * Per-round score weight (classic doubling); the third-place playoff is a bonus
 * game weighted like a semifinal. Index by `roundOf(game)`.
 */
export const ROUND_WEIGHT: ReadonlyArray<number> = [1, 2, 4, 8, 16, 8];

/** Sentinel for an undecided game result in the oracle. */
export const UNDECIDED = 255;

/** The score weight for a game index, derived from its round. */
export function roundWeight(game: number): number {
    return ROUND_WEIGHT[roundOf(game)];
}

/**
 * Sums a bracket's weighted score against the decided games in `results`,
 * mirroring the on-chain `score_bracket`. A game contributes its round weight
 * only when its result is decided (not {@link UNDECIDED}) and the pick matches.
 */
export function scoreBracket(picks: ReadonlyArray<number>, results: ReadonlyArray<number>): number {
    if (picks.length !== GAME_COUNT || results.length !== GAME_COUNT) {
        throw new RangeError(`expected ${GAME_COUNT} picks and results`);
    }
    let score = 0;
    for (let g = 0; g < GAME_COUNT; g++) {
        if (results[g] !== UNDECIDED && picks[g] === results[g]) {
            score += roundWeight(g);
        }
    }
    return score;
}

/** Absolute difference between a tiebreaker guess and the actual Round-of-32 goal total. */
export function closeness(guess: number, actual: number): number {
    return Math.abs(guess - actual);
}
