import assert from 'node:assert/strict';
import { describe, test } from 'node:test';

import { GAME_COUNT, randomBracket } from '../src/bracket.ts';
import { closeness, ROUND_WEIGHT, roundWeight, scoreBracket, UNDECIDED } from '../src/scoring.ts';

describe('ROUND_WEIGHT', () => {
    test('matches the on-chain per-round weights', () => {
        assert.deepEqual(ROUND_WEIGHT, [1, 2, 4, 8, 16, 8]);
    });

    test('roundWeight derives a game weight from its round', () => {
        assert.equal(roundWeight(0), 1);
        assert.equal(roundWeight(16), 2);
        assert.equal(roundWeight(24), 4);
        assert.equal(roundWeight(28), 8);
        assert.equal(roundWeight(30), 16);
        assert.equal(roundWeight(31), 8);
    });
});

describe('scoreBracket', () => {
    test('a perfect bracket scores the full weight sum', () => {
        const { picks } = randomBracket();
        let expected = 0;
        for (let g = 0; g < GAME_COUNT; g++) expected += roundWeight(g);
        assert.equal(scoreBracket(picks, picks), expected);
    });

    test('ignores games still UNDECIDED in the oracle', () => {
        const { picks } = randomBracket();
        const results = picks.slice();
        results[0] = UNDECIDED;
        results[30] = UNDECIDED;
        const expected = scoreBracket(picks, picks) - roundWeight(0) - roundWeight(30);
        assert.equal(scoreBracket(picks, results), expected);
    });

    test('counts only matching decided picks', () => {
        const picks = new Array<number>(GAME_COUNT).fill(UNDECIDED);
        const results = new Array<number>(GAME_COUNT).fill(UNDECIDED);
        picks[0] = 0;
        results[0] = 0; // match, R32 weight 1
        picks[16] = 0;
        results[16] = 1; // decided but mismatched
        assert.equal(scoreBracket(picks, results), 1);
    });

    test('returns zero when nothing matches', () => {
        const { picks } = randomBracket();
        const results = picks.map(p => (p === 0 ? 1 : 0));
        assert.equal(scoreBracket(picks, results), 0);
    });

    test('returns zero when every result is undecided', () => {
        const { picks } = randomBracket();
        const results = new Array<number>(GAME_COUNT).fill(UNDECIDED);
        assert.equal(scoreBracket(picks, results), 0);
    });

    test('throws when picks or results are not full-length', () => {
        const full = new Array<number>(GAME_COUNT).fill(UNDECIDED);
        assert.throws(() => scoreBracket([], full), RangeError);
        assert.throws(() => scoreBracket(full, []), RangeError);
    });
});

describe('closeness', () => {
    test('is the absolute difference between guess and actual', () => {
        assert.equal(closeness(10, 14), 4);
        assert.equal(closeness(14, 10), 4);
        assert.equal(closeness(7, 7), 0);
    });
});
