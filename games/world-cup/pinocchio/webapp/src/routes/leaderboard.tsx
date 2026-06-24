import { useWallet } from '@solana/connector/react';
import { buildLeaderboard, type LeaderboardEntry, type Oracle } from '@solana/world-cup';
import { FlaskConical, Trophy } from 'lucide-react';
import { useEffect, useMemo, useState } from 'react';
import { useNavigate } from 'react-router';

import { BracketDisplaySkeleton } from '@/components/bracket/bracket-display';
import { BracketSearch } from '@/components/bracket/bracket-search';
import { Flag } from '@/components/bracket/flag';
import { displayTeam } from '@/components/bracket/teams';
import { LoadError } from '@/components/load-error';
import { Button } from '@/components/ui/button';
import { useLeaderboard } from '@/hooks/use-leaderboard';
import { cn, ellipsify } from '@/lib/utils';

/** How many ranked rows to show before pinning just the connected wallet's row. */
const TOP_N = 50;

/** When `VITE_PREVIEW=true`, allow previewing the leaderboard with synthetic results. */
const PREVIEW_ENABLED = import.meta.env.VITE_PREVIEW === 'true';

export function Leaderboard() {
    const { account } = useWallet();
    const leaderboard = useLeaderboard();

    // Preview a synthetic, fully-decided tournament so scores are visible before any
    // real results are posted. The demo oracle is imported lazily so it never ships
    // unless previewing.
    const [demo, setDemo] = useState(PREVIEW_ENABLED);
    const [demoOracle, setDemoOracle] = useState<Oracle | null>(null);
    useEffect(() => {
        if (!demo || !PREVIEW_ENABLED) return;
        let active = true;
        void import('@/components/bracket/demo-oracle').then(m => {
            if (active) setDemoOracle(m.demoLeaderboardOracle());
        });
        return () => {
            active = false;
        };
    }, [demo]);

    const data = leaderboard.data;
    const previewing = demo && PREVIEW_ENABLED;
    const oracle = previewing ? demoOracle : (data?.oracle ?? null);
    const entries = useMemo(() => (data ? buildLeaderboard(data.brackets, oracle) : []), [data, oracle]);

    if (leaderboard.isLoading) {
        return <BracketDisplaySkeleton />;
    }

    if (leaderboard.isError || !data) {
        return (
            <LoadError
                title="Couldn’t load the leaderboard"
                body="We hit a network error reaching the cluster. Try again."
                onRetry={() => void leaderboard.refetch()}
            />
        );
    }

    const hasResults = previewing ? oracle != null : data.hasResults;

    if (entries.length === 0) {
        return (
            <div className="mx-auto flex w-full max-w-xl flex-col items-center gap-4 py-16 text-center">
                <div className="flex size-14 items-center justify-center rounded-2xl bg-secondary text-muted-foreground">
                    <Trophy className="size-7" />
                </div>
                <h1 className="text-2xl font-bold tracking-tight">No brackets yet</h1>
                <p className="text-muted-foreground">Once entrants submit brackets, the leaderboard fills in here.</p>
                <BracketSearch className="mt-2" />
            </div>
        );
    }

    const top = entries.slice(0, TOP_N);
    const myIndex = account ? entries.findIndex(e => e.owner === account) : -1;
    const pinned = myIndex >= TOP_N ? entries[myIndex] : null;

    return (
        <div className="mx-auto w-full max-w-3xl">
            <header className="mb-6 flex items-center gap-4">
                <div className="flex size-12 shrink-0 items-center justify-center rounded-xl bg-primary text-primary-foreground">
                    <Trophy className="size-6" />
                </div>
                <div className="min-w-0 flex-1">
                    <h1 className="text-2xl font-bold tracking-tight">Leaderboard</h1>
                    <p className="text-sm text-muted-foreground">
                        {hasResults
                            ? `${entries.length} ${entries.length === 1 ? 'bracket' : 'brackets'}, ranked by ${previewing ? 'preview' : 'live'} score`
                            : 'Scores update live as results post — everyone starts at 0'}
                    </p>
                </div>
                {PREVIEW_ENABLED && (
                    <Button
                        variant={demo ? 'default' : 'secondary'}
                        size="sm"
                        onClick={() => setDemo(d => !d)}
                        title="Preview the leaderboard with synthetic results (VITE_PREVIEW)"
                    >
                        <FlaskConical className="size-4" />
                        {demo ? 'Live' : 'Preview'}
                    </Button>
                )}
            </header>

            <div className="overflow-hidden rounded-2xl border bg-card shadow-sm">
                <div className="grid grid-cols-[2.5rem_1fr_auto] items-center gap-3 border-b bg-secondary/40 px-4 py-2.5 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
                    <span>#</span>
                    <span>Bracket</span>
                    <span className="text-right">Score</span>
                </div>
                <div className="divide-y">
                    {top.map(entry => (
                        <Row key={entry.address} entry={entry} isMe={entry.owner === account} />
                    ))}
                </div>
                {pinned && (
                    <div className="divide-y border-t-2 border-primary/30">
                        <Row entry={pinned} isMe />
                    </div>
                )}
            </div>

            <p className="mt-3 text-center text-xs text-muted-foreground">Tap any row to open that bracket.</p>
        </div>
    );
}

function Row({ entry, isMe }: { entry: LeaderboardEntry; isMe: boolean }) {
    const navigate = useNavigate();
    const champion = displayTeam(entry.championSlot);

    return (
        <button
            type="button"
            onClick={() => navigate(`/b/${entry.owner}`)}
            className={cn(
                'grid w-full grid-cols-[2.5rem_1fr_auto] items-center gap-3 px-4 py-3 text-left transition-colors hover:bg-secondary/50',
                isMe && 'bg-primary/10 hover:bg-primary/15',
            )}
        >
            <span className="font-mono text-sm font-semibold tabular-nums text-muted-foreground">{entry.rank}</span>

            <span className="flex min-w-0 items-center gap-3">
                <Flag code={champion.code} title={champion.name} className="h-7 w-[42px] shrink-0" />
                <span className="min-w-0">
                    <span className="flex items-center gap-2">
                        <span className="truncate font-mono text-sm font-medium">{ellipsify(entry.owner, 4)}</span>
                        {isMe && (
                            <span className="shrink-0 rounded bg-primary px-1.5 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-primary-foreground">
                                You
                            </span>
                        )}
                    </span>
                    <span className="mt-1 flex flex-col gap-0.5 text-xs text-muted-foreground">
                        <PodiumPick place={1} slot={entry.championSlot} />
                        <PodiumPick place={2} slot={entry.runnerUpSlot} />
                        <PodiumPick place={3} slot={entry.thirdPlaceSlot} />
                    </span>
                </span>
            </span>

            <span className="text-right">
                <span className="block font-mono text-lg font-bold tabular-nums leading-none">{entry.score}</span>
                {entry.closeness != null && (
                    <span className="mt-1 block text-[11px] text-muted-foreground">off by {entry.closeness}</span>
                )}
            </span>
        </button>
    );
}

/** One of a bracket's predicted podium finishers: place number and team name. */
function PodiumPick({ place, slot }: { place: number; slot: number }) {
    const team = displayTeam(slot);
    return (
        <span className="flex min-w-0 items-center gap-1.5">
            <span className="w-2 shrink-0 font-mono text-[10px] text-muted-foreground/60">{place}</span>
            <span className="truncate">{team.name}</span>
        </span>
    );
}
