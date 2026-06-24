/**
 * Demo: submits a full, consistency-checked 32-game bracket on-chain.
 *
 * Real-world teams are mapped onto the program's positional team ids (0..31).
 * The bracket is simulated by team strength, with one thumb on the scale: the
 * USA wins every game it plays and lifts the trophy. 🇺🇸
 *
 * Env:
 *   ENTRANT - entrant keypair secret as a JSON [u8] array (defaults to ADMIN)
 *   ADMIN   - admin keypair secret as a JSON [u8] array (fallback entrant)
 *   RPC_URL - cluster endpoint (default: http://127.0.0.1:8899)
 */

import { createClient, createKeyPairSignerFromBytes } from '@solana/kit';
import { solanaRpc } from '@solana/kit-plugin-rpc';
import { signer } from '@solana/kit-plugin-signer';
import { getSubmitBracketInstructionAsync } from '@solana/world-cup';

const RPC_URL = process.env.RPC_URL ?? 'http://127.0.0.1:8899';

const THIRD_PLACE_GAME = 31;
const USA = 'USA';

/**
 * 32 teams in seeded slot order (slot 0 is the strongest bracket position).
 * The USA sits in slot 0 so its path to the final is the top line of the tree.
 * `strength` breaks every other game; higher wins.
 */
const TEAMS: { name: string; strength: number }[] = [
    { name: USA, strength: 999 }, // slot 0 — host, destined champion
    { name: 'Ghana', strength: 71 },
    { name: 'Argentina', strength: 96 },
    { name: 'Australia', strength: 74 },
    { name: 'France', strength: 95 },
    { name: 'Ecuador', strength: 77 },
    { name: 'England', strength: 92 },
    { name: 'Senegal', strength: 80 },
    { name: 'Brazil', strength: 94 },
    { name: 'South Korea', strength: 76 },
    { name: 'Portugal', strength: 91 },
    { name: 'Japan', strength: 81 },
    { name: 'Spain', strength: 93 },
    { name: 'Morocco', strength: 84 },
    { name: 'Netherlands', strength: 90 },
    { name: 'Mexico', strength: 79 },
    { name: 'Germany', strength: 89 },
    { name: 'Uruguay', strength: 82 },
    { name: 'Belgium', strength: 88 },
    { name: 'Switzerland', strength: 78 },
    { name: 'Croatia', strength: 87 },
    { name: 'Denmark', strength: 83 },
    { name: 'Italy', strength: 86 },
    { name: 'Canada', strength: 72 },
    { name: 'Colombia', strength: 85 },
    { name: 'Poland', strength: 73 },
    { name: 'Norway', strength: 80 },
    { name: 'Nigeria', strength: 75 },
    { name: 'Austria', strength: 79 },
    { name: 'Serbia', strength: 74 },
    { name: 'Ivory Coast', strength: 70 },
    { name: 'Saudi Arabia', strength: 68 }, // slot 31
];

const FINAL_GAME = 30;
const usaSlot = TEAMS.findIndex(t => t.name === USA);

/** Team in a slot; throws on an out-of-range id so the result is never undefined. */
function team(slot: number): { name: string; strength: number } {
    const t = TEAMS[slot];
    if (!t) throw new Error(`no team in slot ${slot}`);
    return t;
}

/** Two R32 slots contesting game `g` (0..15). */
const r32Slots = (g: number): [number, number] => [2 * g, 2 * g + 1];

/** Two feeder games for a non-leaf knockout game (16..30). */
const children = (g: number): [number, number] => [(g - 16) * 2, (g - 16) * 2 + 1];

/** The team that should win a contest between two slots: USA always, else strength. */
function winner(a: number, b: number): number {
    if (a === usaSlot || b === usaSlot) return usaSlot;
    return team(a).strength >= team(b).strength ? a : b;
}

/** Winning slot of any knockout game (0..30), computed down the tree. */
function advance(game: number): number {
    if (game < 16) {
        const [a, b] = r32Slots(game);
        return winner(a, b);
    }
    const [c0, c1] = children(game);
    return winner(advance(c0), advance(c1));
}

/** Loser of a semifinal: the feeder winner the semifinal did not advance. */
function semifinalLoser(sf: number): number {
    const [c0, c1] = children(sf);
    const w0 = advance(c0);
    const w1 = advance(c1);
    return winner(w0, w1) === w0 ? w1 : w0;
}

/** Builds a fully consistent bracket where the USA wins it all. */
function buildPicks(): number[] {
    const picks: number[] = [];
    for (let g = 0; g <= FINAL_GAME; g++) picks.push(advance(g));
    picks[THIRD_PLACE_GAME] = winner(semifinalLoser(28), semifinalLoser(29));
    return picks;
}

function entrantSecret(): Uint8Array {
    const raw = process.env.ENTRANT ?? process.env.ADMIN;
    if (!raw) throw new Error('ENTRANT (or ADMIN) env var is required (JSON [u8] array of the keypair secret)');
    return Uint8Array.from(JSON.parse(raw) as number[]);
}

async function main() {
    const entrant = await createKeyPairSignerFromBytes(entrantSecret());
    const client = createClient()
        .use(signer(entrant))
        .use(solanaRpc({ rpcUrl: RPC_URL }));

    const picks = buildPicks();
    const tiebreakerGuess = 84; // predicted Round-of-32 total goals

    const champion = advance(FINAL_GAME);
    console.log(`Entrant: ${entrant.address}`);
    console.log(`RPC:     ${RPC_URL}`);
    console.log(`Champion: ${team(champion).name} 🏆`);
    console.log(`Final:    ${team(champion).name} def. ${team(semifinalLoser(29)).name}`);
    console.log(`3rd:      ${team(winner(semifinalLoser(28), semifinalLoser(29))).name}`);
    console.log(`Tiebreaker (R32 goals): ${tiebreakerGuess}`);

    const instruction = await getSubmitBracketInstructionAsync({
        entrant,
        submitBracketData: { picks, tiebreakerGuess },
    });

    const { context } = await client.sendTransaction([instruction]);
    console.log(`Bracket submitted: ${context.signature}`);
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
