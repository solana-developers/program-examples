import { children, r32Slots, Round, ROUND_WEIGHT, THIRD_PLACE_GAME } from '@solana/world-cup';

/** A single bracket slot: a chosen team id, or `null` when not yet picked. */
export type Pick = number | null;

/** Static metadata for one round of the knockout bracket. */
export interface RoundMeta {
    /** Game indices that belong to this round. */
    games: number[];
    /** Full label, e.g. `"Round of 16"`. */
    label: string;
    round: Round;
    /** Compact label for tabs, e.g. `"R16"`. */
    short: string;
    /** Points a correct pick in this round is worth. */
    weight: number;
}

function range(lo: number, hi: number): number[] {
    return Array.from({ length: hi - lo + 1 }, (_, i) => lo + i);
}

/** The six rounds in play order, each with its game indices and score weight. */
export const ROUNDS: readonly RoundMeta[] = [
    { games: range(0, 15), label: 'Round of 32', round: Round.R32, short: 'R32', weight: ROUND_WEIGHT[Round.R32] },
    { games: range(16, 23), label: 'Round of 16', round: Round.R16, short: 'R16', weight: ROUND_WEIGHT[Round.R16] },
    { games: range(24, 27), label: 'Quarterfinals', round: Round.Qf, short: 'QF', weight: ROUND_WEIGHT[Round.Qf] },
    { games: [28, 29], label: 'Semifinals', round: Round.Sf, short: 'SF', weight: ROUND_WEIGHT[Round.Sf] },
    { games: [30], label: 'Final', round: Round.Final, short: 'Final', weight: ROUND_WEIGHT[Round.Final] },
    {
        games: [THIRD_PLACE_GAME],
        label: 'Third place',
        round: Round.ThirdPlace,
        short: '3rd',
        weight: ROUND_WEIGHT[Round.ThirdPlace],
    },
];

/** The losing team of a semifinal given current picks, or `null` if undecided. */
function semifinalLoser(semifinal: number, picks: readonly Pick[]): Pick {
    const [a, b] = children(semifinal);
    const winnerA = picks[a];
    const winnerB = picks[b];
    const winner = picks[semifinal];
    if (winnerA == null || winnerB == null || winner == null) return null;
    return winner === winnerA ? winnerB : winnerA;
}

/** Game indices of the two semifinals. */
const SEMIFINAL_GAMES = ROUNDS.find(r => r.round === Round.Sf)!.games;

/** The two third-place contestants (the semifinal losers), each `null` until known. */
export function thirdPlaceContestants(picks: readonly Pick[]): [Pick, Pick] {
    const [sfA, sfB] = SEMIFINAL_GAMES;
    return [semifinalLoser(sfA, picks), semifinalLoser(sfB, picks)];
}

/**
 * The two teams contesting a game given current picks. Round-of-32 slots are
 * fixed by position; later rounds resolve to the feeder winners (or `null` until
 * those feeders are decided); the third-place game resolves to the semifinal losers.
 */
export function competitorsOf(game: number, picks: readonly Pick[]): [Pick, Pick] {
    if (game <= 15) {
        const [a, b] = r32Slots(game);
        return [a, b];
    }
    if (game === THIRD_PLACE_GAME) return thirdPlaceContestants(picks);
    const [c0, c1] = children(game);
    return [picks[c0] ?? null, picks[c1] ?? null];
}
