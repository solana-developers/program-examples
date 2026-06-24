/** @jsxImportSource react */
import { type ReactNode } from 'react';
import { ImageResponse } from '@vercel/og';
import { address, createSolanaRpc } from '@solana/kit';
import {
    FINAL_GAME,
    GAME_COUNT,
    TEAM_NAMES,
    THIRD_PLACE_GAME,
    children,
    fetchMaybeBracketFromSeeds,
} from '@solana/world-cup';

import { displayTeam } from '../src/components/bracket/teams';
import { PROJECT_HANDLE, SHARE_HASHTAG, bracketHighlights, flagEmoji } from '../src/lib/share';

export const config = { runtime: 'edge' };

const WIDTH = 1200;
const HEIGHT = 630;
const RPC_URL = process.env.OG_RPC_URL ?? 'https://api.devnet.solana.com';

/** A bracket's podium never changes once submitted, so the rendered card caches hard at the edge. */
const CACHE_HEADERS = { 'cache-control': 'public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800' };

const INK = '#0A0A0F';
const SOLANA_PURPLE = '#9945FF';
const SOLANA_GREEN = '#14F195';
const GOLD = '#F5C451';

/**
 * `@vercel/og` ships a single Geist weight, so `fontWeight` alone renders flat.
 * Loading static-weight Inter from a CDN restores real typographic hierarchy.
 * Each fetch is independent and best-effort: any failure drops that weight and
 * the card still renders with whatever loaded (including none).
 */
const FONT_URLS: ReadonlyArray<{ weight: 400 | 600 | 800; url: string }> = [
    { weight: 400, url: 'https://cdn.jsdelivr.net/fontsource/fonts/inter@latest/latin-400-normal.ttf' },
    { weight: 600, url: 'https://cdn.jsdelivr.net/fontsource/fonts/inter@latest/latin-600-normal.ttf' },
    { weight: 800, url: 'https://cdn.jsdelivr.net/fontsource/fonts/inter@latest/latin-800-normal.ttf' },
];

type LoadedFont = { name: 'Inter'; data: ArrayBuffer; weight: 400 | 600 | 800; style: 'normal' };

async function loadFonts(): Promise<LoadedFont[]> {
    const results = await Promise.all(
        FONT_URLS.map(async ({ weight, url }) => {
            try {
                const res = await fetch(url);
                if (!res.ok) return null;
                const data = await res.arrayBuffer();
                return { name: 'Inter', data, weight, style: 'normal' } satisfies LoadedFont;
            } catch {
                return null;
            }
        }),
    );
    return results.filter((f): f is LoadedFont => f !== null);
}

/** A polished placeholder pill for a slot that has no decided nation yet. */
function PlaceholderChip({ code, size }: { code: string; size: number }) {
    return (
        <div
            style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                minWidth: size * 1.5,
                height: size,
                padding: '0 16px',
                borderRadius: 10,
                background: 'rgba(255,255,255,0.05)',
                border: '1px dashed rgba(255,255,255,0.2)',
                fontSize: size * 0.42,
                fontWeight: 700,
                letterSpacing: 1,
                color: 'rgba(255,255,255,0.55)',
            }}
        >
            {code}
        </div>
    );
}

function Header() {
    return (
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
                <div
                    style={{
                        display: 'flex',
                        width: 14,
                        height: 38,
                        borderRadius: 7,
                        background: `linear-gradient(180deg, ${SOLANA_PURPLE} 0%, ${SOLANA_GREEN} 100%)`,
                    }}
                />
                <span style={{ fontSize: 25, fontWeight: 800, letterSpacing: 5, color: '#FFFFFF' }}>
                    2026 WORLD CUP
                </span>
                <span style={{ fontSize: 25, fontWeight: 600, letterSpacing: 5, color: 'rgba(255,255,255,0.42)' }}>
                    BRACKET
                </span>
            </div>
            <span style={{ fontSize: 24, fontWeight: 600, color: 'rgba(255,255,255,0.5)' }}>{PROJECT_HANDLE}</span>
        </div>
    );
}

function ChampionRow({ slot }: { slot: number }) {
    const { name } = displayTeam(slot);
    const emoji = flagEmoji(slot);
    return (
        <div style={{ display: 'flex', alignItems: 'center', gap: 28 }}>
            <div
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    width: 116,
                    height: 116,
                    borderRadius: 28,
                    background: 'rgba(245,196,81,0.12)',
                    border: `2px solid rgba(245,196,81,0.55)`,
                }}
            >
                <span style={{ fontSize: 70, lineHeight: 1 }}>{emoji || '🏆'}</span>
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                <span style={{ fontSize: 22, fontWeight: 800, letterSpacing: 3, color: GOLD }}>CHAMPION</span>
                <span style={{ fontSize: 96, fontWeight: 800, lineHeight: 1, color: '#FFFFFF' }}>
                    {emoji ? name : 'To be decided'}
                </span>
            </div>
        </div>
    );
}

function MinorRow({ medal, label, slot }: { medal: string; label: string; slot: number }) {
    const { name } = displayTeam(slot);
    const emoji = flagEmoji(slot);
    return (
        <div style={{ display: 'flex', alignItems: 'center', gap: 18 }}>
            <span style={{ fontSize: 40, lineHeight: 1 }}>{medal}</span>
            <span
                style={{ fontSize: 18, fontWeight: 700, letterSpacing: 2, color: 'rgba(255,255,255,0.4)', width: 150 }}
            >
                {label}
            </span>
            {emoji ? (
                <>
                    <span style={{ fontSize: 44, lineHeight: 1 }}>{emoji}</span>
                    <span style={{ fontSize: 44, fontWeight: 600, color: 'rgba(255,255,255,0.92)' }}>{name}</span>
                </>
            ) : (
                <PlaceholderChip code={name} size={44} />
            )}
        </div>
    );
}

function Card({ children }: { children: ReactNode }) {
    return (
        <div
            style={{
                position: 'relative',
                width: WIDTH,
                height: HEIGHT,
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'space-between',
                padding: 64,
                color: '#FFFFFF',
                fontFamily: 'Inter',
                background: INK,
            }}
        >
            <div
                style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    width: WIDTH,
                    height: 6,
                    background: `linear-gradient(90deg, ${SOLANA_PURPLE} 0%, ${SOLANA_GREEN} 100%)`,
                }}
            />
            <div
                style={{
                    position: 'absolute',
                    top: -180,
                    right: -160,
                    width: 620,
                    height: 620,
                    borderRadius: 620,
                    background: `radial-gradient(circle, rgba(153,69,255,0.22) 0%, rgba(10,10,15,0) 70%)`,
                }}
            />
            <div
                style={{
                    position: 'absolute',
                    bottom: -220,
                    left: -120,
                    width: 560,
                    height: 560,
                    borderRadius: 560,
                    background: `radial-gradient(circle, rgba(20,241,149,0.12) 0%, rgba(10,10,15,0) 70%)`,
                }}
            />
            {children}
        </div>
    );
}

function Footer({ label }: { label: string }) {
    return (
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <span style={{ fontSize: 25, fontWeight: 600, color: 'rgba(255,255,255,0.55)' }}>{label}</span>
            <div
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    padding: '10px 22px',
                    borderRadius: 999,
                    border: '1px solid rgba(255,255,255,0.16)',
                    background: 'rgba(255,255,255,0.04)',
                    fontSize: 22,
                    fontWeight: 700,
                    letterSpacing: 1,
                    color: 'rgba(255,255,255,0.9)',
                }}
            >
                {SHARE_HASHTAG}
            </div>
        </div>
    );
}

function podiumImage(picks: ReadonlyArray<number>, fonts: LoadedFont[]): ImageResponse {
    const { championSlot, runnerUpSlot, thirdSlot } = bracketHighlights(picks);
    return new ImageResponse(
        <Card>
            <Header />
            <div style={{ display: 'flex', flexDirection: 'column', gap: 40 }}>
                <ChampionRow slot={championSlot} />
                <div style={{ display: 'flex', flexDirection: 'column', gap: 22 }}>
                    <MinorRow medal="🥈" label="RUNNER-UP" slot={runnerUpSlot} />
                    <MinorRow medal="🥉" label="THIRD" slot={thirdSlot} />
                </div>
            </div>
            <Footer label="My picks" />
        </Card>,
        { width: WIDTH, height: HEIGHT, fonts, headers: CACHE_HEADERS },
    );
}

function fallbackImage(fonts: LoadedFont[]): ImageResponse {
    return new ImageResponse(
        <Card>
            <Header />
            <div style={{ display: 'flex', flexDirection: 'column', gap: 18, maxWidth: 880 }}>
                <span style={{ fontSize: 26, fontWeight: 800, letterSpacing: 3, color: GOLD }}>32 KNOCKOUT GAMES</span>
                <span style={{ fontSize: 104, fontWeight: 800, lineHeight: 1, color: '#FFFFFF' }}>
                    Call the bracket 🏆
                </span>
                <span style={{ fontSize: 32, fontWeight: 600, color: 'rgba(255,255,255,0.62)' }}>
                    Your picks, settled on-chain. Winner takes the pot.
                </span>
            </div>
            <Footer label="Make your picks" />
        </Card>,
        { width: WIDTH, height: HEIGHT, fonts, headers: CACHE_HEADERS },
    );
}

/** A sample podium from the live nations, so the card renders without an RPC call. */
function demoPicks(): number[] {
    const slot = (name: string) => Math.max(0, TEAM_NAMES.indexOf(name));
    const picks = new Array<number>(GAME_COUNT).fill(0);
    const [semiA, semiB] = children(FINAL_GAME);
    picks[semiA] = slot('USA');
    picks[semiB] = slot('Germany');
    picks[FINAL_GAME] = slot('Germany');
    picks[THIRD_PLACE_GAME] = slot('Mexico');
    return picks;
}

export default async function handler(request: Request): Promise<Response> {
    const fonts = await loadFonts();
    const params = new URL(request.url).searchParams;
    if (params.has('demo')) return podiumImage(demoPicks(), fonts);
    const owner = params.get('b');
    if (!owner) return fallbackImage(fonts);
    try {
        const rpc = createSolanaRpc(RPC_URL);
        const bracket = await fetchMaybeBracketFromSeeds(rpc, { owner: address(owner) });
        if (!bracket.exists) return fallbackImage(fonts);
        return podiumImage(bracket.data.picks, fonts);
    } catch {
        return fallbackImage(fonts);
    }
}
