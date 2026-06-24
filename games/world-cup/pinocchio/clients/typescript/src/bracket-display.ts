import { FINAL_GAME, GAME_COUNT, Round, roundOf } from './bracket.js';
import { TEAM_NAMES, teamName } from './teams.js';

/** Human-readable label for each {@link Round}. */
const ROUND_LABELS: Readonly<Record<Round, string>> = {
    [Round.R32]: 'Round of 32',
    [Round.R16]: 'Round of 16',
    [Round.Qf]: 'Quarterfinal',
    [Round.Sf]: 'Semifinal',
    [Round.Final]: 'Final',
    [Round.ThirdPlace]: 'Third-place playoff',
};

/** One game's resolved outcome in a bracket, ready for display. */
export type BracketGameRow = {
    /** Game index `0..31`. */
    game: number;
    /** The round this game belongs to. */
    round: Round;
    /** Human-readable round label, e.g. `"Round of 16"`. */
    roundLabel: string;
    /** Resolved display name of the picked winner. */
    winnerName: string;
    /** Positional slot of the picked winner. */
    winnerSlot: number;
};

/**
 * Expands a bracket's 32 picks into a per-game breakdown: round, round label,
 * winning team slot, and resolved team name for every game index `0..31`.
 * Pass `names` to override the default {@link TEAM_NAMES} placeholders.
 */
export function bracketRows(picks: ReadonlyArray<number>, names: ReadonlyArray<string> = TEAM_NAMES): BracketGameRow[] {
    if (picks.length !== GAME_COUNT) {
        throw new RangeError(`expected ${GAME_COUNT} picks, got ${picks.length}`);
    }
    return picks.map((winnerSlot, game) => {
        const round = roundOf(game);
        return {
            game,
            round,
            roundLabel: ROUND_LABELS[round],
            winnerName: teamName(winnerSlot, names),
            winnerSlot,
        };
    });
}

/** The picked champion: the Final (game 30) winner's slot and resolved name. */
export type Champion = { name: string; slot: number };

/**
 * The picked champion — the winner of the Final (game `30`). Pass `names` to
 * override the default {@link TEAM_NAMES} placeholders.
 */
export function champion(picks: ReadonlyArray<number>, names: ReadonlyArray<string> = TEAM_NAMES): Champion {
    if (picks.length !== GAME_COUNT) {
        throw new RangeError(`expected ${GAME_COUNT} picks, got ${picks.length}`);
    }
    const slot = picks[FINAL_GAME];
    return { name: teamName(slot, names), slot };
}
