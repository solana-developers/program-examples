import { children, GAME_COUNT, r32Slots, TEAM_COUNT, THIRD_PLACE_GAME, thirdPlaceSlots } from './bracket.js';

/**
 * Edition-specific bracket labels for the FIFA World Cup 2026 knockout stage.
 *
 * The program is label-agnostic — games and team slots are just positional ids
 * `0..31`. FIFA, by contrast, names matches `M73..M104` (scheduling order, not
 * bracket order) and seeds slots from group positions (`1C`, `2F`, `3ABCDF`, …).
 * This module is the single source of truth tying those official labels onto the
 * internal indices, chosen so the program's fixed `children()` adjacency
 * reproduces the real bracket. NEXT-TOURNAMENT REPLACEMENT POINT: swap this file.
 */

/** Internal game index `0..31` → official FIFA match label (`"M74"`, …). */
export const MATCH_LABELS: readonly string[] = [
    'M74',
    'M77',
    'M73',
    'M75',
    'M83',
    'M84',
    'M81',
    'M82', // R32, games 0..7
    'M76',
    'M78',
    'M79',
    'M80',
    'M86',
    'M88',
    'M85',
    'M87', // R32, games 8..15
    'M89',
    'M90',
    'M93',
    'M94',
    'M91',
    'M92',
    'M95',
    'M96', // R16, games 16..23
    'M97',
    'M98',
    'M99',
    'M100', // Quarterfinals, games 24..27
    'M101',
    'M102', // Semifinals, games 28..29
    'M104', // Final, game 30
    'M103', // Third-place playoff, game 31
];

/**
 * Kickoff instant per game index as a UTC ISO-8601 timestamp, mirroring the
 * official bracket. Stored as absolute instants (not wall-clock strings) so each
 * client can render a match in the viewer's own local timezone. The 2026
 * tournament spans five venue timezones — PT, MT, CT, ET, and Mexico's UTC-6,
 * which (unlike US Central) does not observe DST — so a bare date/time would be
 * ambiguous. Comments note the venue-local kickoff each instant was derived from.
 */
export const MATCH_KICKOFFS: readonly string[] = [
    '2026-06-29T20:30:00Z', // 0  M74  Boston 16:30 ET
    '2026-06-30T21:00:00Z', // 1  M77  NY/NJ 17:00 ET
    '2026-06-28T19:00:00Z', // 2  M73  Los Angeles 12:00 PT
    '2026-06-30T01:00:00Z', // 3  M75  Monterrey 19:00 (UTC-6)
    '2026-07-02T23:00:00Z', // 4  M83  Toronto 19:00 ET
    '2026-07-02T19:00:00Z', // 5  M84  Los Angeles 12:00 PT
    '2026-07-02T00:00:00Z', // 6  M81  SF Bay 17:00 PT
    '2026-07-01T20:00:00Z', // 7  M82  Seattle 13:00 PT
    '2026-06-29T17:00:00Z', // 8  M76  Houston 12:00 CT
    '2026-06-30T17:00:00Z', // 9  M78  Dallas 12:00 CT
    '2026-07-01T01:00:00Z', // 10 M79  Mexico City 19:00 (UTC-6)
    '2026-07-01T16:00:00Z', // 11 M80  Atlanta 12:00 ET
    '2026-07-03T22:00:00Z', // 12 M86  Miami 18:00 ET
    '2026-07-03T18:00:00Z', // 13 M88  Dallas 13:00 CT
    '2026-07-03T03:00:00Z', // 14 M85  Vancouver 20:00 PT
    '2026-07-04T01:30:00Z', // 15 M87  Kansas City 20:30 CT
    '2026-07-04T21:00:00Z', // 16 M89  Philadelphia 17:00 ET
    '2026-07-04T17:00:00Z', // 17 M90  Houston 12:00 CT
    '2026-07-06T19:00:00Z', // 18 M93  Dallas 14:00 CT
    '2026-07-07T00:00:00Z', // 19 M94  Seattle 17:00 PT
    '2026-07-05T20:00:00Z', // 20 M91  NY/NJ 16:00 ET
    '2026-07-06T00:00:00Z', // 21 M92  Mexico City 18:00 (UTC-6)
    '2026-07-07T16:00:00Z', // 22 M95  Atlanta 12:00 ET
    '2026-07-07T20:00:00Z', // 23 M96  Vancouver 13:00 PT
    '2026-07-09T20:00:00Z', // 24 M97  Boston 16:00 ET
    '2026-07-10T19:00:00Z', // 25 M98  Los Angeles 12:00 PT
    '2026-07-11T21:00:00Z', // 26 M99  Miami 17:00 ET
    '2026-07-12T01:00:00Z', // 27 M100 Kansas City 20:00 CT
    '2026-07-14T19:00:00Z', // 28 M101 Dallas 14:00 CT
    '2026-07-15T19:00:00Z', // 29 M102 Atlanta 15:00 ET
    '2026-07-19T19:00:00Z', // 30 M104 NY/NJ 15:00 ET (Final)
    '2026-07-18T21:00:00Z', // 31 M103 Miami 17:00 ET (Third place)
];

/**
 * Team slot `0..31` → seeded label. Already-qualified teams (Germany, USA,
 * Mexico) are concrete; the rest are group-position placeholders (`1C` = winner
 * of Group C, `2F` = runner-up of Group F, `3ABCDF` = third place from one of
 * those groups) until the draw fills them. Slot order is bracket order: this is
 * what makes the internal tree reproduce FIFA's matchups, so do not reorder.
 */
export const SLOT_LABELS: readonly string[] = [
    'Germany',
    '3ABCDF',
    '1I',
    '3CDFGH',
    '2A',
    '2B',
    '1F',
    '2C', // slots 0..7
    '2K',
    '2L',
    '1H',
    '2J',
    'USA',
    '3BEFIJ',
    '1G',
    '3AEHIJ', // slots 8..15
    '1C',
    '2F',
    '2E',
    '2I',
    'Mexico',
    '3CEFHI',
    '1L',
    '3EHIJK', // slots 16..23
    'Argentina',
    '2H',
    '2D',
    '2G',
    '1B',
    '3EFGIJ',
    '1K',
    '3DEIJL', // slots 24..31
];

const GAME_INDEX_BY_LABEL: ReadonlyMap<string, number> = new Map(MATCH_LABELS.map((label, index) => [label, index]));

/** The official match label for a game index `0..31`. Throws when out of range. */
export function matchLabel(game: number): string {
    const label = MATCH_LABELS[game];
    if (label === undefined) throw new RangeError(`game ${game} out of range 0..${GAME_COUNT}`);
    return label;
}

/** The game index `0..31` for an official match label (`"M74"`). Throws when unknown. */
export function gameIndexOf(label: string): number {
    const game = GAME_INDEX_BY_LABEL.get(label);
    if (game === undefined) throw new RangeError(`unknown match label ${label}`);
    return game;
}

/** A match's two competitors as display labels: team slots for the Round of 32,
 * feeder-match references (`"W74"`) for later rounds, and the semifinal
 * runners-up (`"RU101"`) for the third-place playoff. */
export function contestantsOf(game: number): [string, string] {
    if (game < 0 || game >= GAME_COUNT) throw new RangeError(`game ${game} out of range 0..${GAME_COUNT}`);
    if (game < 16) {
        const [a, b] = r32Slots(game);
        return [SLOT_LABELS[a], SLOT_LABELS[b]];
    }
    if (game === THIRD_PLACE_GAME) {
        return [`RU${matchLabel(28).slice(1)}`, `RU${matchLabel(29).slice(1)}`];
    }
    const [c0, c1] = children(game);
    return [`W${matchLabel(c0).slice(1)}`, `W${matchLabel(c1).slice(1)}`];
}

/**
 * Translates an official result ("Germany won M74") into the `{ game, winner }`
 * payload for a `post_result` instruction, guarding that `winnerSlot` is actually
 * a competitor of that match. Round-of-32 matches validate against their two team
 * slots directly. Later rounds need the oracle's current `results` (slot per game,
 * {@link UNDECIDED} for unposted) to know who reached the match — pass it to get
 * the same contestant check the program enforces; omit it to validate range only.
 */
export function resultForMatch(
    label: string,
    winnerSlot: number,
    results?: ReadonlyArray<number>,
): { game: number; winner: number } {
    const game = gameIndexOf(label);
    if (!Number.isInteger(winnerSlot) || winnerSlot < 0 || winnerSlot >= TEAM_COUNT) {
        throw new RangeError(`winner slot ${winnerSlot} out of range 0..${TEAM_COUNT}`);
    }
    if (game < 16) {
        const [a, b] = r32Slots(game);
        if (winnerSlot !== a && winnerSlot !== b) {
            throw new Error(`${label} is contested by slots ${a} and ${b}, not ${winnerSlot}`);
        }
    } else if (results !== undefined) {
        const [c0, c1] = game === THIRD_PLACE_GAME ? thirdPlaceSlots(results) : pickFeederWinners(game, results);
        if (winnerSlot !== c0 && winnerSlot !== c1) {
            throw new Error(`${label} is contested by slots ${c0} and ${c1}, not ${winnerSlot}`);
        }
    }
    return { game, winner: winnerSlot };
}

function pickFeederWinners(game: number, results: ReadonlyArray<number>): [number, number] {
    const [c0, c1] = children(game);
    return [results[c0], results[c1]];
}

/**
 * Asserts the FIFA label tables are internally consistent: the match labels are a
 * bijection over `M73..M104`, every slot label is distinct, and the official
 * feeder structure (transcribed in {@link FIFA_FEEDERS}) matches the program's
 * `children()` adjacency through {@link MATCH_LABELS}. Throws on any drift so a
 * mistyped label or a reordered slot is caught instead of silently producing the
 * wrong matchups.
 */
export function assertFifaScheduleConsistent(): void {
    if (MATCH_LABELS.length !== GAME_COUNT) throw new Error(`expected ${GAME_COUNT} match labels`);
    if (SLOT_LABELS.length !== TEAM_COUNT) throw new Error(`expected ${TEAM_COUNT} slot labels`);
    if (MATCH_KICKOFFS.length !== GAME_COUNT) throw new Error(`expected ${GAME_COUNT} match kickoffs`);
    for (const iso of MATCH_KICKOFFS) {
        if (Number.isNaN(Date.parse(iso))) throw new Error(`invalid kickoff timestamp: ${iso}`);
    }

    const numbers = MATCH_LABELS.map(label => Number(label.slice(1)));
    const unique = new Set(numbers);
    if (unique.size !== GAME_COUNT) throw new Error('match labels are not unique');
    for (let m = 73; m <= 104; m++) {
        if (!unique.has(m)) throw new Error(`missing match M${m}`);
    }
    if (new Set(SLOT_LABELS).size !== TEAM_COUNT) throw new Error('slot labels are not unique');

    for (const [label, feeders] of FIFA_FEEDERS) {
        const game = gameIndexOf(label);
        const [d0, d1] = game === THIRD_PLACE_GAME ? [matchLabel(28), matchLabel(29)] : children(game).map(matchLabel);
        const derived = new Set([d0, d1]);
        if (new Set(feeders).size !== 2 || derived.size !== 2 || !feeders.every(f => derived.has(f))) {
            throw new Error(`feeders for ${label} should be ${feeders.join(', ')} but resolved to ${d0}, ${d1}`);
        }
    }
}

/**
 * Official feeder matches for each non-Round-of-32 match (`[match, [feederA,
 * feederB]]`), transcribed directly from the bracket independent of the internal
 * topology. {@link assertFifaScheduleConsistent} checks the index mapping
 * reproduces these.
 */
export const FIFA_FEEDERS: readonly [string, readonly [string, string]][] = [
    ['M89', ['M74', 'M77']],
    ['M90', ['M73', 'M75']],
    ['M91', ['M76', 'M78']],
    ['M92', ['M79', 'M80']],
    ['M93', ['M83', 'M84']],
    ['M94', ['M81', 'M82']],
    ['M95', ['M86', 'M88']],
    ['M96', ['M85', 'M87']],
    ['M97', ['M89', 'M90']],
    ['M98', ['M93', 'M94']],
    ['M99', ['M91', 'M92']],
    ['M100', ['M95', 'M96']],
    ['M101', ['M97', 'M98']],
    ['M102', ['M99', 'M100']],
    ['M104', ['M101', 'M102']],
    ['M103', ['M101', 'M102']],
];
