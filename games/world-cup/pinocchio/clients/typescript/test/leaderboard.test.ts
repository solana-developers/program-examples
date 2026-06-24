import assert from 'node:assert/strict';
import { describe, test } from 'node:test';

import type { Address } from '@solana/kit';

import { children, FINAL_GAME, randomBracket, THIRD_PLACE_GAME } from '../src/bracket.ts';
import type { Oracle } from '../src/generated/index.ts';
import { buildLeaderboard, type BracketAccount } from '../src/leaderboard.ts';

function bracketAccount(owner: string, picks: ReadonlyArray<number>, tiebreakerGuess = 0): BracketAccount {
    return {
        address: `${owner}-pda` as Address,
        data: {
            bump: 255,
            discriminator: 2,
            owner: owner as Address,
            picks: [...picks],
            score: 0,
            tallyMask: 0,
            tiebreakerGuess,
        },
    };
}

function makeOracle(
    results: ReadonlyArray<number>,
    opts: { goalsPosted?: boolean; totalGoalsR32?: number } = {},
): Oracle {
    return {
        bump: 255,
        decidedMask: 0,
        discriminator: 1,
        goalsPosted: opts.goalsPosted ? 1 : 0,
        results: [...results],
        totalGoalsR32: opts.totalGoalsR32 ?? 0,
    };
}

describe('buildLeaderboard', () => {
    test('ranks by live score descending', () => {
        const { picks } = randomBracket();
        const perfect = picks.slice();
        const missedFinal = picks.slice();
        missedFinal[30] = (picks[30] + 1) % 32; // wrong final pick → loses the 16-pt game
        const allWrong = picks.map(p => (p + 1) % 32);

        const entries = buildLeaderboard(
            [bracketAccount('C', allWrong), bracketAccount('A', perfect), bracketAccount('B', missedFinal)],
            makeOracle(picks),
        );

        assert.deepEqual(
            entries.map(e => e.owner),
            ['A', 'B', 'C'],
        );
        assert.deepEqual(
            entries.map(e => e.rank),
            [1, 2, 3],
        );
        assert.equal(entries[0].score, 88);
        assert.equal(entries[1].score, 72);
        assert.equal(entries[2].score, 0);
    });

    test('equal score and closeness share a rank (1, 1, 3)', () => {
        const { picks } = randomBracket();
        const entries = buildLeaderboard(
            [
                bracketAccount('A', picks, 10),
                bracketAccount('B', picks, 10),
                bracketAccount(
                    'C',
                    picks.map(p => (p + 1) % 32),
                    10,
                ),
            ],
            makeOracle(picks, { goalsPosted: true, totalGoalsR32: 14 }),
        );

        assert.deepEqual(
            entries.map(e => e.rank),
            [1, 1, 3],
        );
        // Same score + same closeness for A and B.
        assert.equal(entries[0].closeness, 4);
        assert.equal(entries[1].closeness, 4);
    });

    test('derives the podium slots (1st/2nd/3rd) from the picks', () => {
        const { picks } = randomBracket();
        const [{ championSlot, runnerUpSlot, thirdPlaceSlot }] = buildLeaderboard([bracketAccount('A', picks)], null);

        const [sfA, sfB] = children(FINAL_GAME); // the two semifinals feeding the Final
        assert.equal(championSlot, picks[FINAL_GAME]);
        assert.equal(thirdPlaceSlot, picks[THIRD_PLACE_GAME]);
        // The runner-up is the finalist that isn't the champion.
        assert.deepEqual([championSlot, runnerUpSlot].sort(), [picks[sfA], picks[sfB]].sort());
        assert.notEqual(championSlot, runnerUpSlot);
    });

    test('closeness is null until Round-of-32 goals are posted', () => {
        const { picks } = randomBracket();
        const entries = buildLeaderboard([bracketAccount('A', picks, 10)], makeOracle(picks));
        assert.equal(entries[0].closeness, null);
    });

    test('breaks score ties by closeness ascending', () => {
        const { picks } = randomBracket();
        const entries = buildLeaderboard(
            [bracketAccount('A', picks, 99), bracketAccount('B', picks, 13)],
            makeOracle(picks, { goalsPosted: true, totalGoalsR32: 14 }),
        );
        // B's guess (13) is closer to 14 than A's (99), so B ranks first despite equal score.
        assert.deepEqual(
            entries.map(e => e.owner),
            ['B', 'A'],
        );
        assert.deepEqual(
            entries.map(e => e.rank),
            [1, 2],
        );
    });

    test('with no oracle, every entry scores 0 and shares the top rank', () => {
        const { picks } = randomBracket();
        const entries = buildLeaderboard([bracketAccount('A', picks, 5), bracketAccount('B', picks, 9)], null);
        assert.deepEqual(
            entries.map(e => e.score),
            [0, 0],
        );
        assert.deepEqual(
            entries.map(e => e.rank),
            [1, 1],
        );
        assert.deepEqual(
            entries.map(e => e.closeness),
            [null, null],
        );
    });
});
