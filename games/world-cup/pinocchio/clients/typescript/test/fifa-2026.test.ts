import assert from 'node:assert/strict';
import { describe, test } from 'node:test';

import { children, GAME_COUNT, r32Slots, TEAM_COUNT } from '../src/bracket.ts';
import {
    assertFifaScheduleConsistent,
    contestantsOf,
    FIFA_FEEDERS,
    gameIndexOf,
    MATCH_LABELS,
    matchLabel,
    resultForMatch,
    SLOT_LABELS,
} from '../src/fifa-2026.ts';
import { TEAM_NAMES } from '../src/teams.ts';

describe('match labels', () => {
    test('are a bijection over M73..M104', () => {
        assert.equal(MATCH_LABELS.length, GAME_COUNT);
        const numbers = MATCH_LABELS.map(label => Number(label.slice(1)));
        assert.equal(new Set(numbers).size, GAME_COUNT);
        for (let m = 73; m <= 104; m++) {
            assert.ok(numbers.includes(m), `missing M${m}`);
        }
    });

    test('matchLabel and gameIndexOf round-trip for every game', () => {
        for (let g = 0; g < GAME_COUNT; g++) {
            assert.equal(gameIndexOf(matchLabel(g)), g);
        }
    });

    test('the final is M104 and the third-place playoff is M103', () => {
        assert.equal(matchLabel(30), 'M104');
        assert.equal(matchLabel(31), 'M103');
    });

    test('throw on out-of-range index and unknown label', () => {
        assert.throws(() => matchLabel(32));
        assert.throws(() => gameIndexOf('M999'));
    });
});

describe('schedule consistency', () => {
    test('assertFifaScheduleConsistent passes', () => {
        assert.doesNotThrow(assertFifaScheduleConsistent);
    });

    test('the index mapping reproduces the official feeder structure', () => {
        for (const [label, feeders] of FIFA_FEEDERS) {
            const game = gameIndexOf(label);
            const derived = game === 31 ? [matchLabel(28), matchLabel(29)] : children(game).map(matchLabel);
            assert.deepEqual(new Set(derived), new Set(feeders), `feeders for ${label}`);
        }
    });
});

describe('slot labels', () => {
    test('are 32 distinct entries', () => {
        assert.equal(SLOT_LABELS.length, TEAM_COUNT);
        assert.equal(new Set(SLOT_LABELS).size, TEAM_COUNT);
    });

    test('are all group-position seeds (1=winner, 2=runner-up, 3=best third)', () => {
        for (const label of SLOT_LABELS) {
            assert.match(label, /^[123][A-L]+$/, `${label} is not a group-position seed`);
        }
        // group winners the example bracket fills with host/seed nations
        assert.equal(SLOT_LABELS[0], '1E'); // Germany
        assert.equal(SLOT_LABELS[12], '1D'); // USA (FIFA host position D1)
        assert.equal(SLOT_LABELS[20], '1A'); // Mexico (FIFA host position A1)
        assert.equal(SLOT_LABELS[24], '1J'); // Argentina
    });

    test('TEAM_NAMES resolves every slot to a distinct real team', () => {
        assert.equal(TEAM_NAMES.length, TEAM_COUNT);
        assert.equal(new Set(TEAM_NAMES).size, TEAM_COUNT);
    });
});

describe('final-bracket seeding', () => {
    // Each slot's nation and the FIFA group-position seed it occupies, transcribed
    // from the final bracket / FIFA regulations Annex (e.g. USA won Group D, so it
    // is the `1D` seed). Kept independent of TEAM_NAMES and SLOT_LABELS so a
    // mis-edit of either array is caught here.
    const EXPECTED: ReadonlyArray<readonly [team: string, seed: string]> = [
        ['Germany', '1E'],
        ['Paraguay', '3ABCDF'],
        ['France', '1I'],
        ['Sweden', '3CDFGH'],
        ['South Africa', '2A'],
        ['Canada', '2B'],
        ['Netherlands', '1F'],
        ['Morocco', '2C'],
        ['Portugal', '2K'],
        ['Croatia', '2L'],
        ['Spain', '1H'],
        ['Austria', '2J'],
        ['USA', '1D'],
        ['Bosnia & Herzegovina', '3BEFIJ'],
        ['Belgium', '1G'],
        ['Senegal', '3AEHIJ'],
        ['Brazil', '1C'],
        ['Japan', '2F'],
        ["Côte d'Ivoire", '2E'],
        ['Norway', '2I'],
        ['Mexico', '1A'],
        ['Ecuador', '3CEFHI'],
        ['England', '1L'],
        ['DR Congo', '3EHIJK'],
        ['Argentina', '1J'],
        ['Cape Verde', '2H'],
        ['Australia', '2D'],
        ['Egypt', '2G'],
        ['Switzerland', '1B'],
        ['Algeria', '3EFGIJ'],
        ['Colombia', '1K'],
        ['Ghana', '3DEIJL'],
    ];

    test('the expected table covers all 32 slots', () => {
        assert.equal(EXPECTED.length, TEAM_COUNT);
    });

    test('each slot places the right nation in its FIFA seed', () => {
        EXPECTED.forEach(([team, seed], slot) => {
            assert.equal(TEAM_NAMES[slot], team, `nation at slot ${slot}`);
            assert.equal(SLOT_LABELS[slot], seed, `seed at slot ${slot}`);
        });
    });
});

describe('contestantsOf', () => {
    test('Round-of-32 games show their two team slots', () => {
        const [a, b] = r32Slots(0);
        assert.deepEqual(contestantsOf(0), [SLOT_LABELS[a], SLOT_LABELS[b]]);
        assert.deepEqual(contestantsOf(0), ['1E', '3ABCDF']);
    });

    test('later rounds show feeder-match winners', () => {
        assert.deepEqual(contestantsOf(16), ['W74', 'W77']); // M89
        assert.deepEqual(contestantsOf(30), ['W101', 'W102']); // M104 final
    });

    test('the third-place playoff shows the semifinal runners-up', () => {
        assert.deepEqual(contestantsOf(31), ['RU101', 'RU102']); // M103
    });

    test('throws on out-of-range game indices', () => {
        assert.throws(() => contestantsOf(-1));
        assert.throws(() => contestantsOf(32));
    });
});

describe('resultForMatch', () => {
    test('translates a Round-of-32 winner to a post_result payload', () => {
        assert.deepEqual(resultForMatch('M74', 0), { game: 0, winner: 0 });
        assert.deepEqual(resultForMatch('M74', 1), { game: 0, winner: 1 });
    });

    test('rejects a winner that does not contest the match', () => {
        assert.throws(() => resultForMatch('M74', 2)); // M74 is slots 0,1
    });

    test('rejects an out-of-range winner slot', () => {
        assert.throws(() => resultForMatch('M74', 99));
    });

    test('validates later rounds against the oracle results when provided', () => {
        const results = new Array<number>(GAME_COUNT).fill(0);
        results[0] = 1; // winner of game 0 (feeder of M89/game 16) is slot 1
        results[1] = 2; // winner of game 1 is slot 2
        assert.deepEqual(resultForMatch('M89', 1, results), { game: 16, winner: 1 });
        assert.deepEqual(resultForMatch('M89', 2, results), { game: 16, winner: 2 });
        assert.throws(() => resultForMatch('M89', 3, results)); // 3 did not reach M89
    });
});
