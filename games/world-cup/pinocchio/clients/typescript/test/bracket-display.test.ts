import assert from 'node:assert/strict';
import { describe, test } from 'node:test';

import { GAME_COUNT, randomBracket, Round, roundOf } from '../src/bracket.ts';
import { bracketRows, champion } from '../src/bracket-display.ts';
import { teamName, TEAM_NAMES } from '../src/teams.ts';

describe('teams', () => {
    test('TEAM_NAMES has 32 entries resolved to the final bracket teams', () => {
        assert.equal(TEAM_NAMES.length, 32);
        assert.equal(TEAM_NAMES[0], 'Germany');
        assert.equal(TEAM_NAMES[31], 'Ghana');
    });

    test('teamName resolves a slot and honors a custom names override', () => {
        assert.equal(teamName(0), 'Germany');
        assert.equal(teamName(5, ['A', 'B', 'C', 'D', 'E', 'USA']), 'USA');
    });

    test('teamName throws RangeError on out-of-range slots', () => {
        assert.throws(() => teamName(-1), RangeError);
        assert.throws(() => teamName(32), RangeError);
        assert.throws(() => teamName(1.5), RangeError);
    });
});

describe('bracket-display', () => {
    test('champion matches picks[30]', () => {
        const { picks } = randomBracket();
        const c = champion(picks);
        assert.equal(c.slot, picks[30]);
        assert.equal(c.name, TEAM_NAMES[picks[30]]);
    });

    test('champion honors a custom names override', () => {
        const { picks } = randomBracket();
        const names = Array.from({ length: 32 }, (_, i) => `X${i}`);
        assert.equal(champion(picks, names).name, names[picks[30]]);
    });

    test('bracketRows returns 32 rows with correct round and resolved names', () => {
        const { picks } = randomBracket();
        const rows = bracketRows(picks);
        assert.equal(rows.length, GAME_COUNT);
        for (const sample of [0, 15, 16, 23, 24, 27, 28, 29, 30, 31]) {
            const row = rows[sample];
            assert.equal(row.game, sample);
            assert.equal(row.round, roundOf(sample));
            assert.equal(row.winnerSlot, picks[sample]);
            assert.equal(row.winnerName, TEAM_NAMES[picks[sample]]);
        }
        assert.equal(rows[30].round, Round.Final);
        assert.equal(rows[31].round, Round.ThirdPlace);
        assert.equal(rows[30].roundLabel, 'Final');
        assert.equal(rows[31].roundLabel, 'Third-place playoff');
    });

    test('bracketRows honors a custom names override', () => {
        const { picks } = randomBracket();
        const names = Array.from({ length: 32 }, (_, i) => `Squad ${i}`);
        const rows = bracketRows(picks, names);
        assert.equal(rows[0].winnerName, names[picks[0]]);
    });
});
