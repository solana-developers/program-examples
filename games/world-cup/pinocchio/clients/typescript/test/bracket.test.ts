import assert from 'node:assert/strict';
import { describe, test } from 'node:test';

import { createNoopSigner } from '@solana/kit';

import {
    children,
    GAME_COUNT,
    isValidBracket,
    parseSubmitBracketInstruction,
    r32Slots,
    randomBracket,
    Round,
    roundOf,
    THIRD_PLACE_GAME,
    thirdPlaceSlots,
    validateBracket,
    yolo,
} from '../src/index.ts';

describe('topology', () => {
    test('roundOf maps game indices to rounds', () => {
        assert.equal(roundOf(0), Round.R32);
        assert.equal(roundOf(15), Round.R32);
        assert.equal(roundOf(16), Round.R16);
        assert.equal(roundOf(23), Round.R16);
        assert.equal(roundOf(24), Round.Qf);
        assert.equal(roundOf(27), Round.Qf);
        assert.equal(roundOf(28), Round.Sf);
        assert.equal(roundOf(29), Round.Sf);
        assert.equal(roundOf(30), Round.Final);
        assert.equal(roundOf(31), Round.ThirdPlace);
    });

    test('r32Slots returns the two team slots of a Round-of-32 game', () => {
        assert.deepEqual(r32Slots(0), [0, 1]);
        assert.deepEqual(r32Slots(1), [2, 3]);
        assert.deepEqual(r32Slots(15), [30, 31]);
    });

    test('children returns the two feeder games of a knockout game', () => {
        assert.deepEqual(children(16), [0, 1]);
        assert.deepEqual(children(30), [28, 29]);
    });

    test('thirdPlaceSlots returns the two semifinal losers', () => {
        const slots = new Array<number>(GAME_COUNT).fill(0);
        slots[24] = 1;
        slots[25] = 2;
        slots[26] = 3;
        slots[27] = 4;
        slots[28] = 1; // SF 28 winner is 1 -> loser is 2
        slots[29] = 4; // SF 29 winner is 4 -> loser is 3
        assert.deepEqual(thirdPlaceSlots(slots), [2, 3]);
    });
});

describe('validateBracket', () => {
    test('accepts a randomly generated bracket', () => {
        assert.equal(validateBracket(randomBracket().picks).ok, true);
    });

    test('rejects a bracket of the wrong length', () => {
        assert.deepEqual(validateBracket([1, 2, 3]), {
            ok: false,
            game: -1,
            reason: `expected ${GAME_COUNT} picks, got 3`,
        });
    });

    test('rejects an out-of-range team', () => {
        const picks = randomBracket().picks;
        picks[0] = 99;
        const result = validateBracket(picks);
        assert.equal(result.ok, false);
        assert.equal(result.ok === false && result.game, 0);
    });

    test('rejects a team that does not play in a Round-of-32 game', () => {
        const picks = randomBracket().picks;
        picks[1] = 5; // game 1 contests slots 2 and 3
        const result = validateBracket(picks);
        assert.equal(result.ok, false);
        assert.equal(result.ok === false && result.game, 1);
    });

    test('rejects a winner not advanced from its feeders', () => {
        const picks = new Array<number>(GAME_COUNT);
        for (let g = 0; g < 16; g++) picks[g] = r32Slots(g)[0];
        for (let g = 16; g <= 30; g++) picks[g] = picks[children(g)[0]]!;
        picks[THIRD_PLACE_GAME] = thirdPlaceSlots(picks)[0];
        picks[16] = picks[2]!; // not a feeder winner of game 16 (feeders 0,1)
        const result = validateBracket(picks);
        assert.equal(result.ok, false);
        assert.equal(result.ok === false && result.game, 16);
    });
});

describe('randomBracket', () => {
    test('always produces a valid bracket', () => {
        for (let i = 0; i < 1000; i++) {
            assert.equal(isValidBracket(randomBracket().picks), true);
        }
    });

    test('honors a fixed tiebreaker guess', () => {
        assert.equal(randomBracket(42).tiebreakerGuess, 42);
    });

    test('generates a tiebreaker in range when unspecified', () => {
        const { tiebreakerGuess } = randomBracket();
        assert.ok(tiebreakerGuess >= 0 && tiebreakerGuess <= 200);
    });
});

describe('yolo', () => {
    test('builds a submitBracket instruction with a valid random bracket', async () => {
        const entrant = createNoopSigner('11111111111111111111111111111112');
        const instruction = await yolo({ entrant });
        const parsed = parseSubmitBracketInstruction(instruction);
        assert.equal(isValidBracket(parsed.data.submitBracketData.picks), true);
        assert.equal(parsed.accounts.entrant.address, entrant.address);
    });
});
