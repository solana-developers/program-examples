import { children, FINAL_GAME, THIRD_PLACE_GAME } from '@solana/world-cup';
import getCountryFlag from 'country-flag-icons/unicode';

import { displayTeam, teamHandle } from '../components/bracket/teams';

/** The X handle the game tags in every shared post. */
export const PROJECT_HANDLE = '@SolanaFndn';

/** The hashtag appended to every shared post. */
export const SHARE_HASHTAG = '#WorldCupSolana';

/** The three podium slots derived from a bracket's picks. */
export interface Podium {
    championSlot: number;
    runnerUpSlot: number;
    thirdSlot: number;
}

/** Champion (Final winner), runner-up (the losing finalist), and third-place winner. */
export function bracketHighlights(picks: ReadonlyArray<number>): Podium {
    const championSlot = picks[FINAL_GAME];
    const [finalA, finalB] = children(FINAL_GAME);
    const runnerUpSlot = picks[finalA] === championSlot ? picks[finalB] : picks[finalA];
    return { championSlot, runnerUpSlot, thirdSlot: picks[THIRD_PLACE_GAME] };
}

/** A flag emoji for a team slot, or an empty string when the slot has no alpha-2 code. */
export function flagEmoji(slot: number): string {
    const { code } = displayTeam(slot);
    return code && /^[A-Z]{2}$/.test(code) ? getCountryFlag(code) : '';
}

function podiumLine(medal: string, slot: number): string {
    const emoji = flagEmoji(slot);
    return `${medal} ${emoji ? `${emoji} ` : ''}${displayTeam(slot).name}`;
}

/** The drafted post body for a bracket — champion + runner-up + third, then tags. */
export function tweetText(picks: ReadonlyArray<number>): string {
    const { championSlot, runnerUpSlot, thirdSlot } = bracketHighlights(picks);
    const champHandle = teamHandle(championSlot);
    const tags = [champHandle, SHARE_HASHTAG, PROJECT_HANDLE].filter(Boolean).join(' ');
    return [
        'Locked my 2026 World Cup bracket 🏆',
        '',
        podiumLine('🥇', championSlot),
        podiumLine('🥈', runnerUpSlot),
        podiumLine('🥉', thirdSlot),
        '',
        'Think you can do better? Make your picks:',
        '',
        tags,
    ].join('\n');
}

/** The public, shareable URL for one wallet's bracket. */
export function bracketShareUrl(origin: string, owner: string): string {
    return `${origin}/b/${owner}`;
}

/** An `x.com` intent URL that opens a pre-drafted post with the bracket link attached. */
export function xIntentUrl({ text, url }: { text: string; url: string }): string {
    const params = new URLSearchParams({ text, url });
    return `https://twitter.com/intent/tweet?${params.toString()}`;
}
