import { useState } from 'react';
import {
    getSubmitBracketInstructionAsync,
    randomBracket,
    Round,
    validateBracket,
    type SubmitBracketDataArgs,
} from '@solana/world-cup';
import { useKitTransactionSigner } from '@solana/connector/react';
import { useQueryClient } from '@tanstack/react-query';
import { Link } from 'react-router';
import { CheckCircle2, Check, Lock, Trophy } from 'lucide-react';
import { toast } from 'sonner';

import { Button } from '@/components/ui/button';
import { ActionBar, type BracketPending } from '@/components/bracket/action-bar';
import { useWalletTransactionSignAndSend } from '@/components/solana/use-wallet-transaction-sign-and-send';
import { useTransactionToast } from '@/components/use-transaction-toast';
import { bracketQueryKey } from '@/hooks/use-bracket';
import { useClusterConfig } from '@/hooks/use-cluster-config';
import { useTournament } from '@/hooks/use-tournament';
import { formatSol } from '@/lib/format';
import { Flag } from '@/components/bracket/flag';
import { MatchupCard } from '@/components/bracket/matchup-card';
import { competitorsOf, ROUNDS, type RoundMeta } from '@/components/bracket/rounds';
import { ShareButton } from '@/components/bracket/share-button';
import { SummaryRail } from '@/components/bracket/summary-rail';
import { displayTeam } from '@/components/bracket/teams';
import { useBracketBuilder } from '@/components/bracket/use-bracket-builder';
import { cn } from '@/lib/utils';

const ROUND_HINT: Record<Round, string> = {
    [Round.R32]: 'Tap the team you think advances out of each tie.',
    [Round.R16]: 'Advance one winner from each Round-of-32 result.',
    [Round.Qf]: 'Send one team through from each Round-of-16 tie.',
    [Round.Sf]: 'Pick the two finalists.',
    [Round.Final]: 'Pick who lifts the trophy.',
    [Round.ThirdPlace]: 'The two beaten semifinalists meet for third place.',
};

export function BracketBuilder() {
    const builder = useBracketBuilder();
    const { picks, setWinner } = builder;
    const [active, setActive] = useState<Round>(Round.R32);
    const [pending, setPending] = useState<BracketPending>(null);
    const [submitted, setSubmitted] = useState<{ address: string; picks: number[] } | null>(null);

    const { data: tournament } = useTournament();
    const entryFee = tournament?.config?.entryFee ?? null;

    const { signer } = useKitTransactionSigner();
    const signAndSend = useWalletTransactionSignAndSend();
    const txToast = useTransactionToast();
    const queryClient = useQueryClient();
    const { id: clusterId } = useClusterConfig();

    function onSubmitted() {
        if (signer) {
            void queryClient.invalidateQueries({ queryKey: bracketQueryKey(clusterId, signer.address) });
        }
    }

    async function submit(data?: SubmitBracketDataArgs) {
        const payload = data ?? builder.toSubmitData();
        if (!payload) {
            toast.error('Pick all 32 winners and a tiebreaker first.');
            return;
        }
        const result = validateBracket(payload.picks);
        if (!result.ok) {
            toast.error(`Bracket is inconsistent at game ${result.game}.`);
            return;
        }
        if (!signer) {
            toast.error('Connect your wallet first.');
            return;
        }
        setPending('submit');
        try {
            const ix = await getSubmitBracketInstructionAsync({ entrant: signer, submitBracketData: payload });
            const sig = await signAndSend(ix, signer);
            setSubmitted({ address: signer.address, picks: payload.picks });
            txToast.onSuccess(sig);
            onSubmitted();
        } catch (err) {
            txToast.onError(err instanceof Error ? err : new Error(String(err)));
        } finally {
            setPending(null);
        }
    }

    async function handleYolo() {
        if (!signer) {
            toast.error('Connect your wallet first.');
            return;
        }
        const data = randomBracket();
        setPending('yolo');
        try {
            const ix = await getSubmitBracketInstructionAsync({ entrant: signer, submitBracketData: data });
            const sig = await signAndSend(ix, signer);
            builder.setBracket(data);
            setActive(Round.Final);
            setSubmitted({ address: signer.address, picks: data.picks });
            txToast.onSuccess(sig);
            onSubmitted();
        } catch (err) {
            txToast.onError(err instanceof Error ? err : new Error(String(err)));
        } finally {
            setPending(null);
        }
    }

    function handleReset() {
        builder.reset();
        setActive(Round.R32);
    }

    const activeMeta = ROUNDS.find(r => r.round === active) ?? ROUNDS[0];

    const railProps = {
        builder,
        entryFee,
        onReset: handleReset,
        onRandom: () => builder.randomize(),
        onYolo: handleYolo,
        onSubmit: () => submit(),
        pending,
    };

    return (
        <div className="mx-auto w-full max-w-6xl pb-28 lg:pb-4">
            <div className="mb-6 flex flex-wrap items-end justify-between gap-3">
                <div>
                    <h1 className="text-2xl font-bold tracking-tight sm:text-3xl">Build your bracket</h1>
                    <p className="mt-1 text-sm text-muted-foreground">
                        Call all 32 knockout games, then guess the Round-of-32 goal total.
                    </p>
                </div>
                <div className="inline-flex items-center gap-2 rounded-full border bg-card px-3 py-1.5 text-sm shadow-sm">
                    <Trophy className="size-4" />
                    <span className="font-medium">2026 World Cup</span>
                    {entryFee != null && (
                        <span className="text-muted-foreground">· {formatSol(entryFee)} SOL entry</span>
                    )}
                </div>
            </div>

            {submitted && (
                <div className="hero-entrance mb-6 flex flex-col gap-4 overflow-hidden rounded-2xl border border-emerald-500/30 bg-emerald-500/10 p-5 sm:flex-row sm:items-center sm:justify-between">
                    <div className="flex items-center gap-4">
                        <div className="flex size-11 shrink-0 items-center justify-center rounded-full bg-emerald-500/15 text-emerald-600 dark:text-emerald-400">
                            <CheckCircle2 className="size-6" />
                        </div>
                        <div>
                            <div className="text-base font-semibold tracking-tight">Bracket locked in</div>
                            <p className="text-sm text-muted-foreground text-pretty">
                                Your picks are in. Post them and see who else dares to call it.
                            </p>
                        </div>
                    </div>
                    <div className="flex shrink-0 items-center gap-2">
                        <Button asChild variant="ghost" size="sm">
                            <Link to={`/b/${submitted.address}`}>View bracket</Link>
                        </Button>
                        <ShareButton owner={submitted.address} picks={submitted.picks} size="sm" label="Share on X" />
                    </div>
                </div>
            )}

            <div className="lg:grid lg:grid-cols-[minmax(0,1fr)_340px] lg:items-start lg:gap-8">
                <div className="min-w-0">
                    <RoundTabs active={active} picks={picks} onSelect={setActive} />

                    <div className="mt-5 mb-4 flex items-baseline justify-between gap-3">
                        <div>
                            <h2 className="text-lg font-semibold tracking-tight">{activeMeta.label}</h2>
                            <p className="text-sm text-muted-foreground">{ROUND_HINT[active]}</p>
                        </div>
                        <span className="shrink-0 rounded-full bg-secondary px-2.5 py-1 font-mono text-xs text-muted-foreground">
                            +{activeMeta.weight} pts
                        </span>
                    </div>

                    <RoundGames meta={activeMeta} picks={picks} onPick={setWinner} />

                    {active === Round.Final && builder.champion != null && (
                        <div className="hero-entrance mt-8 flex flex-col items-center gap-1 text-center">
                            <Trophy className="size-9" />
                            <Flag
                                code={displayTeam(builder.champion).code}
                                title={displayTeam(builder.champion).name}
                                className="h-9 w-[54px]"
                            />
                            <div className="text-xl font-bold">{displayTeam(builder.champion).name}</div>
                            <div className="text-sm text-muted-foreground">Your champion</div>
                        </div>
                    )}

                    <SummaryRail {...railProps} showActions={false} className="mt-8 lg:hidden" />
                </div>

                <SummaryRail {...railProps} showActions className="sticky top-24 hidden lg:flex" />
            </div>

            <div className="fixed inset-x-0 bottom-0 z-30 border-t bg-background/90 px-4 py-3 backdrop-blur lg:hidden">
                <div className="mx-auto max-w-6xl">
                    <ActionBar
                        onReset={handleReset}
                        onRandom={() => builder.randomize()}
                        onYolo={handleYolo}
                        onSubmit={() => submit()}
                        canSubmit={builder.isComplete}
                        canClear={builder.pickCount > 0 || builder.tiebreaker != null}
                        pending={pending}
                    />
                </div>
            </div>
        </div>
    );
}

function RoundTabs({
    active,
    picks,
    onSelect,
}: {
    active: Round;
    picks: readonly (number | null)[];
    onSelect: (round: Round) => void;
}) {
    return (
        <div className="-mx-1 flex gap-1.5 overflow-x-auto px-1 pb-1">
            {ROUNDS.map(meta => {
                const done = meta.games.filter(g => picks[g] != null).length;
                const complete = done === meta.games.length;
                const locked = meta.games.every(g => {
                    const [a, b] = competitorsOf(g, picks);
                    return a == null || b == null;
                });
                const isActive = active === meta.round;
                return (
                    <button
                        key={meta.round}
                        type="button"
                        onClick={() => onSelect(meta.round)}
                        className={cn(
                            'inline-flex shrink-0 items-center gap-2 rounded-full border px-3.5 py-2 text-sm font-medium transition-colors',
                            isActive
                                ? 'border-foreground bg-primary text-primary-foreground'
                                : 'border-transparent bg-secondary text-muted-foreground hover:bg-sand-200 hover:text-foreground',
                        )}
                    >
                        <span>{meta.short}</span>
                        {complete ? (
                            <Check className="size-3.5" />
                        ) : locked ? (
                            <Lock className="size-3.5 opacity-60" />
                        ) : (
                            <span
                                className={cn(
                                    'font-mono text-[11px] tabular-nums',
                                    isActive ? 'opacity-80' : 'opacity-60',
                                )}
                            >
                                {done}/{meta.games.length}
                            </span>
                        )}
                    </button>
                );
            })}
        </div>
    );
}

function RoundGames({
    meta,
    picks,
    onPick,
}: {
    meta: RoundMeta;
    picks: readonly (number | null)[];
    onPick: (game: number, slot: number) => void;
}) {
    if (meta.games.length === 1) {
        const game = meta.games[0];
        return (
            <div className="mx-auto max-w-sm">
                <MatchupCard game={game} picks={picks} onPick={onPick} weight={meta.weight} />
            </div>
        );
    }
    return (
        <div className="grid gap-3 sm:grid-cols-2">
            {meta.games.map(game => (
                <MatchupCard key={game} game={game} picks={picks} onPick={onPick} weight={meta.weight} />
            ))}
        </div>
    );
}
