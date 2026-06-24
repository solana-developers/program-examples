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

    test('place the already-qualified teams', () => {
        assert.equal(SLOT_LABELS[0], 'Germany');
        assert.equal(SLOT_LABELS[12], 'USA');
        assert.equal(SLOT_LABELS[20], 'Mexico');
    });

    test('TEAM_NAMES defaults to the FIFA seeding', () => {
        assert.deepEqual(TEAM_NAMES, SLOT_LABELS);
    });
});

describe('contestantsOf', () => {
    test('Round-of-32 games show their two team slots', () => {
        const [a, b] = r32Slots(0);
        assert.deepEqual(contestantsOf(0), [SLOT_LABELS[a], SLOT_LABELS[b]]);
        assert.deepEqual(contestantsOf(0), ['Germany', '3ABCDF']);
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
