import { getSubmitBracketInstructionAsync, type SubmitBracketAsyncInput } from './generated/index.js';
import type { SubmitBracketDataArgs } from './generated/types/submitBracketData.js';

/** Total games: 31 knockout games + the third-place playoff. */
export const GAME_COUNT = 32;

/** Number of competing teams (positional identities `0..32`). */
export const TEAM_COUNT = 32;

/** Index of the Final; its winner is the bracket's picked champion. */
export const FINAL_GAME = 30;

/** Index of the third-place playoff game. */
export const THIRD_PLACE_GAME = 31;

/** The round a game belongs to, derived from its index. */
export enum Round {
    R32 = 0,
    R16 = 1,
    Qf = 2,
    Sf = 3,
    Final = 4,
    ThirdPlace = 5,
}

/** The round a game index belongs to. */
export function roundOf(game: number): Round {
    if (game <= 15) return Round.R32;
    if (game <= 23) return Round.R16;
    if (game <= 27) return Round.Qf;
    if (game <= 29) return Round.Sf;
    if (game === 30) return Round.Final;
    if (game === 31) return Round.ThirdPlace;
    return Round.Final;
}

/** The two feeder games for a non-leaf knockout game (`g` in `16..=30`). */
export function children(g: number): [number, number] {
    const base = (g - 16) * 2;
    return [base, base + 1];
}

/** The two team slots contesting a Round-of-32 game (`g` in `0..=15`). */
export function r32Slots(g: number): [number, number] {
    return [2 * g, 2 * g + 1];
}

/** The two competitors of the third-place game: the losers of the two semifinals. */
export function thirdPlaceSlots(slots: ReadonlyArray<number>): [number, number] {
    const loser28 = slots[28] === slots[24] ? slots[25] : slots[24];
    const loser29 = slots[29] === slots[26] ? slots[27] : slots[26];
    return [loser28, loser29];
}

/** Outcome of {@link validateBracket}. On failure, `game` is the offending index. */
export type BracketValidation = { game: number; ok: false; reason: string } | { ok: true };

/**
 * Verifies a full bracket is internally consistent, mirroring the on-chain
 * `validate_bracket` check: every pick is a team the bracket itself advanced from
 * one of the game's two feeders. Use before `submitBracket` to reject invalid
 * brackets without a failed transaction.
 */
export function validateBracket(picks: ReadonlyArray<number>): BracketValidation {
    if (picks.length !== GAME_COUNT) {
        return { game: -1, ok: false, reason: `expected ${GAME_COUNT} picks, got ${picks.length}` };
    }
    for (let g = 0; g < GAME_COUNT; g++) {
        const pick = picks[g];
        if (pick < 0 || pick >= TEAM_COUNT) {
            return { game: g, ok: false, reason: `team ${pick} out of range 0..${TEAM_COUNT}` };
        }
        if (g < 16) {
            const [a, b] = r32Slots(g);
            if (pick !== a && pick !== b) {
                return { game: g, ok: false, reason: `team ${pick} does not play in game ${g} (slots ${a}, ${b})` };
            }
        } else if (g <= 30) {
            const [c0, c1] = children(g);
            if (pick !== picks[c0] && pick !== picks[c1]) {
                return { game: g, ok: false, reason: `team ${pick} was not advanced from feeders ${c0}, ${c1}` };
            }
        } else {
            const [l0, l1] = thirdPlaceSlots(picks);
            if (pick !== l0 && pick !== l1) {
                return { game: g, ok: false, reason: `team ${pick} is not a semifinal loser (${l0}, ${l1})` };
            }
        }
    }
    return { ok: true };
}

/** True when `picks` would pass the on-chain consistency check. */
export function isValidBracket(picks: ReadonlyArray<number>): boolean {
    return validateBracket(picks).ok;
}

function pickOne(a: number, b: number): number {
    return Math.random() < 0.5 ? a : b;
}

/**
 * Generates a random internally-consistent bracket. The result always passes
 * {@link validateBracket}. Pass `tiebreakerGuess` to fix the Round-of-32 goal-total
 * guess; otherwise a random value in `0..=200` is used.
 */
export function randomBracket(tiebreakerGuess?: number): SubmitBracketDataArgs {
    const picks = new Array<number>(GAME_COUNT);
    for (let g = 0; g < 16; g++) {
        const [a, b] = r32Slots(g);
        picks[g] = pickOne(a, b);
    }
    for (let g = 16; g <= 30; g++) {
        const [c0, c1] = children(g);
        picks[g] = pickOne(picks[c0], picks[c1]);
    }
    const [l0, l1] = thirdPlaceSlots(picks);
    picks[THIRD_PLACE_GAME] = pickOne(l0, l1);
    return {
        picks,
        tiebreakerGuess: tiebreakerGuess ?? Math.floor(Math.random() * 201),
    };
}

/**
 * Builds a `submitBracket` instruction with a random valid bracket — for testing,
 * demos, or a "feeling lucky" submit. Accepts the same accounts as the generated
 * builder; the bracket data is generated for you.
 */
export function yolo(
    input: Omit<SubmitBracketAsyncInput, 'submitBracketData'> & { tiebreakerGuess?: number },
): ReturnType<typeof getSubmitBracketInstructionAsync> {
    const { tiebreakerGuess, ...accounts } = input;
    return getSubmitBracketInstructionAsync({
        ...accounts,
        submitBracketData: randomBracket(tiebreakerGuess),
    });
}
