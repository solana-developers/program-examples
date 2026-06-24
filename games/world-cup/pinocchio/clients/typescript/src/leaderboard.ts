import {
    type Address,
    type Base58EncodedBytes,
    getBase58Decoder,
    getBase64Encoder,
    type GetProgramAccountsApi,
    type Rpc,
} from '@solana/kit';

import { children, FINAL_GAME, THIRD_PLACE_GAME } from './bracket.js';
import {
    AccountDiscriminator,
    type Bracket,
    getAccountDiscriminatorEncoder,
    getBracketDecoder,
    getBracketEncoder,
    type Oracle,
    WORLD_CUP_PROGRAM_ADDRESS,
} from './generated/index.js';
import { closeness, scoreBracket } from './scoring.js';

/** Byte length of a Bracket account; every account is fixed-size, so this is exact. */
export const BRACKET_ACCOUNT_SIZE = getBracketEncoder().fixedSize;

/** A decoded Bracket account paired with its on-chain address (the bracket PDA). */
export interface BracketAccount {
    address: Address;
    data: Bracket;
}

/** Options for {@link fetchAllBrackets}; defaults target the deployed program. */
export interface FetchAllBracketsConfig {
    /** Override the program to query; defaults to {@link WORLD_CUP_PROGRAM_ADDRESS}. */
    programAddress?: Address;
}

/**
 * The base58 memcmp bytes that select Bracket accounts by their leading
 * discriminator. Derived from the codec rather than hardcoded so it tracks the IDL.
 */
function bracketDiscriminatorFilterBytes(): Base58EncodedBytes {
    const bytes = getAccountDiscriminatorEncoder().encode(AccountDiscriminator.Bracket);
    return getBase58Decoder().decode(bytes) as Base58EncodedBytes;
}

/**
 * Fetches every Bracket account owned by the program in a single `getProgramAccounts`
 * call, narrowed by two TypeSafe filters: the fixed account size and a discriminator
 * memcmp at offset 0. Each account already carries its `picks`, so no follow-up
 * `getMultipleAccounts` is needed to score the field.
 */
export async function fetchAllBrackets(
    rpc: Rpc<GetProgramAccountsApi>,
    config: FetchAllBracketsConfig = {},
): Promise<BracketAccount[]> {
    const programAddress = config.programAddress ?? WORLD_CUP_PROGRAM_ADDRESS;
    const accounts = await rpc
        .getProgramAccounts(programAddress, {
            encoding: 'base64',
            filters: [
                { dataSize: BigInt(BRACKET_ACCOUNT_SIZE) },
                { memcmp: { bytes: bracketDiscriminatorFilterBytes(), encoding: 'base58', offset: 0n } },
            ],
        })
        .send();

    const base64 = getBase64Encoder();
    const decoder = getBracketDecoder();
    return accounts.map(({ account, pubkey }) => ({
        address: pubkey,
        data: decoder.decode(base64.encode(account.data[0])),
    }));
}

/** One ranked row in the leaderboard, scored live against the oracle. */
export interface LeaderboardEntry {
    /** The bracket PDA address. */
    address: Address;
    /** The picked champion's positional team slot — the bracket's 1st place (Final winner). */
    championSlot: number;
    /** `|guess - actual|` once Round-of-32 goals are posted; otherwise `null`. */
    closeness: number | null;
    /** The wallet that submitted the bracket. */
    owner: Address;
    /** 1-based rank; entries with equal `(score, closeness)` share a rank. */
    rank: number;
    /** The picked runner-up's team slot — the bracket's 2nd place (Final loser). */
    runnerUpSlot: number;
    /** Live weighted score against the oracle's decided games; `0` before any results. */
    score: number;
    /** The picked third-place team slot — the bracket's 3rd place (third-place playoff winner). */
    thirdPlaceSlot: number;
    /** The entrant's Round-of-32 total-goals guess. */
    tiebreakerGuess: number;
}

type ScoredEntry = Omit<LeaderboardEntry, 'rank'>;

/** Score desc, then closeness asc (nulls last), then owner asc for a stable order. */
function compareEntries(a: ScoredEntry, b: ScoredEntry): number {
    if (a.score !== b.score) return b.score - a.score;
    if (a.closeness !== b.closeness) {
        if (a.closeness == null) return 1;
        if (b.closeness == null) return -1;
        return a.closeness - b.closeness;
    }
    return a.owner < b.owner ? -1 : a.owner > b.owner ? 1 : 0;
}

/** Two entries tie — and therefore share a rank — when score and closeness match. */
function sameRank(a: ScoredEntry, b: ScoredEntry): boolean {
    return a.score === b.score && a.closeness === b.closeness;
}

/**
 * Ranks brackets by live score, mirroring the on-chain ordering key
 * `(score DESC, closeness ASC)`. Scores are computed against `oracle.results` (so
 * standings are current regardless of whether `refresh_score` has folded a bracket),
 * and closeness is only populated once the oracle has posted Round-of-32 goals. When
 * `oracle` is `null`, every entry scores `0` and shares the top rank. Pure and
 * RPC-free for easy testing.
 */
export function buildLeaderboard(brackets: ReadonlyArray<BracketAccount>, oracle: Oracle | null): LeaderboardEntry[] {
    const goalsPosted = oracle != null && oracle.goalsPosted > 0;
    const [finalistGameA, finalistGameB] = children(FINAL_GAME);

    const scored: ScoredEntry[] = brackets.map(({ address, data }) => {
        const championSlot = data.picks[FINAL_GAME];
        const finalistA = data.picks[finalistGameA];
        return {
            address,
            championSlot,
            closeness: goalsPosted ? closeness(data.tiebreakerGuess, oracle.totalGoalsR32) : null,
            owner: data.owner,
            runnerUpSlot: championSlot === finalistA ? data.picks[finalistGameB] : finalistA,
            score: oracle ? scoreBracket(data.picks, oracle.results) : 0,
            thirdPlaceSlot: data.picks[THIRD_PLACE_GAME],
            tiebreakerGuess: data.tiebreakerGuess,
        };
    });

    scored.sort(compareEntries);

    let rank = 0;
    let prev: ScoredEntry | null = null;
    return scored.map((entry, i) => {
        if (prev == null || !sameRank(prev, entry)) rank = i + 1;
        prev = entry;
        return { rank, ...entry };
    });
}
