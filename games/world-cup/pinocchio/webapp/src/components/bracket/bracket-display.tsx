import { type ReactNode, useEffect, useState } from 'react';
import {
    closeness,
    GAME_COUNT,
    matchLabel,
    scoreBracket,
    UNDECIDED,
    type Bracket,
    type Oracle,
} from '@solana/world-cup';
import { Check, FlaskConical, Medal, Trophy, X } from 'lucide-react';

import { Button } from '@/components/ui/button';
import { shortenAddress } from '@/lib/format';
import { cn } from '@/lib/utils';

import { Flag } from './flag';
import { competitorsOf, ROUNDS, type RoundMeta } from './rounds';
import { ShareButton } from './share-button';
import { displayTeam } from './teams';

interface BracketDisplayProps {
    bracket: Pick<Bracket, 'owner' | 'picks' | 'tiebreakerGuess' | 'score'>;
    /** The oracle, when results have started posting; `null` means picks-only. */
    oracle: Oracle | null;
}

/** When `VITE_PREVIEW=true`, allow previewing a scored bracket from fake results. */
const PREVIEW_ENABLED = import.meta.env.VITE_PREVIEW === 'true';

/** A submitted bracket, read-only. Once the oracle posts results it scores each pick. */
export function BracketDisplay({ bracket, oracle: realOracle }: BracketDisplayProps) {
    const picks = bracket.picks;
    // Preview scored results from fake data; on by default when VITE_PREVIEW=true.
    // The synthetic oracle is loaded lazily so it's never bundled unless previewing.
    const [demo, setDemo] = useState(PREVIEW_ENABLED);
    const [demoOracle, setDemoOracle] = useState<Oracle | null>(null);
    useEffect(() => {
        if (!demo || !PREVIEW_ENABLED) return;
        let active = true;
        void import('./demo-oracle').then(m => {
            if (active) setDemoOracle(m.demoOracleFromPicks(picks));
        });
        return () => {
            active = false;
        };
    }, [demo, picks]);
    const oracle = demo && PREVIEW_ENABLED ? demoOracle : realOracle;
    const champ = displayTeam(picks[30]);
    const decidedCount = oracle ? picks.reduce((n, _, g) => (oracle.results[g] !== UNDECIDED ? n + 1 : n), 0) : 0;
    const correctCount = oracle
        ? picks.reduce((n, pick, g) => (oracle.results[g] !== UNDECIDED && pick === oracle.results[g] ? n + 1 : n), 0)
        : 0;
    const liveScore = oracle ? scoreBracket(picks, oracle.results) : null;
    const goalsPosted = oracle != null && oracle.goalsPosted > 0;

    return (
        <div className="mx-auto w-full max-w-5xl">
            <header className="hero-entrance flex flex-col gap-5 rounded-2xl border bg-card p-6 shadow-sm sm:flex-row sm:items-center sm:justify-between">
                <div className="flex items-center gap-4">
                    <div className="flex size-14 shrink-0 items-center justify-center rounded-xl bg-primary text-primary-foreground">
                        <Trophy className="size-7" />
                    </div>
                    <div className="min-w-0">
                        <div className="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
                            Champion
                        </div>
                        <div className="flex items-center gap-2">
                            <Flag code={champ.code} title={champ.name} className="h-5 w-[30px] shrink-0" />
                            <span className="truncate text-2xl font-bold tracking-tight">{champ.name}</span>
                        </div>
                        <div className="mt-1 font-mono text-xs text-muted-foreground">
                            {shortenAddress(bracket.owner)}
                        </div>
                    </div>
                </div>

                <div className="flex items-center gap-4">
                    {oracle ? (
                        <div className="flex items-center gap-6">
                            <Stat label="Score" value={String(liveScore)} />
                            <Stat label="Correct" value={`${correctCount}/${decidedCount}`} />
                        </div>
                    ) : (
                        <span className="inline-flex items-center gap-2 self-start rounded-full bg-secondary px-3 py-1.5 text-sm text-muted-foreground">
                            Locked in · awaiting results
                        </span>
                    )}
                    <ShareButton owner={bracket.owner} picks={picks} variant="outline" size="sm" label="Share" />
                    {PREVIEW_ENABLED && (
                        <Button
                            variant={demo ? 'default' : 'secondary'}
                            size="sm"
                            onClick={() => setDemo(d => !d)}
                            title="Preview scored results from fake game data (VITE_PREVIEW)"
                        >
                            <FlaskConical className="size-4" />
                            {demo ? 'Live' : 'Preview'}
                        </Button>
                    )}
                </div>
            </header>

            <div className="mt-5 flex items-center justify-between gap-3 rounded-xl border bg-card px-4 py-3 text-sm">
                <span className="text-muted-foreground">Tiebreaker — total goals in the Round of 32</span>
                <span className="font-mono tabular-nums">
                    {bracket.tiebreakerGuess}
                    {goalsPosted && (
                        <span className="ml-2 text-muted-foreground">
                            actual {oracle.totalGoalsR32} · off by{' '}
                            {closeness(bracket.tiebreakerGuess, oracle.totalGoalsR32)}
                        </span>
                    )}
                </span>
            </div>

            <div className="mt-8 space-y-8">
                {ROUNDS.map(meta => (
                    <RoundSection key={meta.round} meta={meta} picks={picks} oracle={oracle} />
                ))}
            </div>
        </div>
    );
}

function Stat({ label, value }: { label: string; value: string }) {
    return (
        <div className="text-right">
            <div className="font-mono text-2xl font-bold tabular-nums leading-none">{value}</div>
            <div className="mt-1 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">{label}</div>
        </div>
    );
}

function RoundSection({ meta, picks, oracle }: { meta: RoundMeta; picks: number[]; oracle: Oracle | null }) {
    const Icon = meta.round === ROUNDS[5].round ? Medal : meta.round === ROUNDS[4].round ? Trophy : null;
    return (
        <section>
            <div className="mb-3 flex items-baseline gap-2">
                {Icon && <Icon className="size-4 self-center text-muted-foreground" />}
                <h2 className="text-lg font-semibold tracking-tight">{meta.label}</h2>
                <span className="font-mono text-xs text-muted-foreground">+{meta.weight} pts</span>
            </div>
            <div className={cn('grid gap-3', meta.games.length > 1 ? 'sm:grid-cols-2' : 'mx-auto max-w-sm')}>
                {meta.games.map(game => (
                    <DisplayMatchup key={game} game={game} picks={picks} oracle={oracle} />
                ))}
            </div>
        </section>
    );
}

function DisplayMatchup({ game, picks, oracle }: { game: number; picks: number[]; oracle: Oracle | null }) {
    const [a, b] = competitorsOf(game, picks);
    const pick = picks[game];
    const result = oracle && oracle.results[game] !== UNDECIDED ? oracle.results[game] : null;

    return (
        <div className="overflow-hidden rounded-xl border bg-card shadow-sm">
            <div className="border-b bg-secondary/40 px-3 py-1.5 text-[11px] font-medium uppercase tracking-wider text-muted-foreground">
                {matchLabel(game)}
            </div>
            <div className="divide-y">
                <TeamRow slot={a} pick={pick} result={result} decided={result != null} />
                <TeamRow slot={b} pick={pick} result={result} decided={result != null} />
            </div>
        </div>
    );
}

interface TeamRowProps {
    slot: number | null;
    pick: number;
    result: number | null;
    decided: boolean;
}

function TeamRow({ slot, pick, result, decided }: TeamRowProps) {
    if (slot == null) {
        return (
            <div className="flex items-center gap-3 px-3 py-2.5 text-sm text-muted-foreground/70">
                <span className="size-7 shrink-0 rounded-md border border-dashed bg-muted/40" />
                <span className="italic">To be decided</span>
            </div>
        );
    }

    const team = displayTeam(slot);
    const isPick = slot === pick;
    const isActual = decided && slot === result;
    const correct = isPick && isActual;
    const wrong = isPick && decided && !isActual;

    return (
        <div
            className={cn(
                'flex items-center gap-3 px-3 py-2.5 text-sm transition-colors',
                correct && 'bg-emerald-500/10 font-semibold text-emerald-700 dark:text-emerald-400',
                wrong && 'bg-destructive/10 text-destructive line-through',
                isPick && !decided && 'bg-primary font-semibold text-primary-foreground',
                !isPick && isActual && 'font-medium',
                !isPick && !isActual && 'text-muted-foreground',
            )}
        >
            <Flag code={team.code} title={team.name} className="h-4 w-6 shrink-0" />
            <span className="flex-1 truncate">{team.name}</span>
            {isPick && correct && <Check className="size-4 shrink-0" />}
            {isPick && wrong && <X className="size-4 shrink-0" />}
            {isPick && !decided && <Check className="size-4 shrink-0" />}
            {!isPick && isActual && (
                <span className="shrink-0 rounded bg-secondary px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
                    Won
                </span>
            )}
        </div>
    );
}

interface BracketDisplaySkeletonProps {
    children?: ReactNode;
}

/** Loading placeholder matching the display's footprint. */
export function BracketDisplaySkeleton({ children }: BracketDisplaySkeletonProps) {
    return (
        <div className="mx-auto w-full max-w-5xl">
            <div className="h-28 animate-pulse rounded-2xl border bg-card" />
            <div className="mt-8 grid gap-3 sm:grid-cols-2">
                {Array.from({ length: Math.min(6, GAME_COUNT) }, (_, i) => (
                    <div key={i} className="h-24 animate-pulse rounded-xl border bg-card" />
                ))}
            </div>
            {children}
        </div>
    );
}
