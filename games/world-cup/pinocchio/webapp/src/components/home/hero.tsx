import { Lock, Sparkles, Trophy } from 'lucide-react';

import { WalletButton } from '@/components/solana/solana-provider';
import { BracketSearch } from '@/components/bracket/bracket-search';
import type { TournamentInfo } from '@/hooks/use-tournament';
import { formatSol } from '@/lib/format';

const STEPS = [
    {
        title: 'Call every game',
        body: 'Pick the winner of all 32 knockout matches, then guess the Round-of-32 goal total as your tiebreaker.',
    },
    {
        title: 'Lock at kickoff',
        body: 'Submit one bracket per wallet for the entry fee. Picks are final the moment the tournament starts.',
    },
    {
        title: 'Highest score sweeps the pot',
        body: 'Each correct pick scores more in later rounds. The single best bracket wins the entire prize pool.',
    },
];

interface HeroProps {
    tournament?: TournamentInfo;
}

/** Disconnected/empty-state landing: what the game is, the live pot, and how to enter. */
export function Hero({ tournament }: HeroProps) {
    const config = tournament?.config ?? null;
    const entrants = config?.entrantCount ?? 0;
    const entryFee = config?.entryFee ?? null;
    const pot = config && entryFee != null ? entryFee * BigInt(entrants) : null;
    const registrationOpen = tournament?.registrationOpen ?? false;

    return (
        <div className="mx-auto w-full max-w-5xl">
            <section className="hero-entrance">
                <div className="inline-flex items-center gap-2 rounded-full border bg-card px-3 py-1.5 text-xs font-medium text-muted-foreground">
                    <Trophy className="size-3.5" />
                    Onchain bracket · 2026 World Cup
                </div>

                <h1 className="mt-5 max-w-3xl text-4xl font-bold leading-[1.05] tracking-tight sm:text-6xl">
                    Predict all 32 knockout games. Win the pot.
                </h1>
                <p className="mt-4 max-w-xl text-lg text-muted-foreground">
                    One bracket per wallet, settled entirely onchain. The sharpest call on the World Cup takes the whole
                    prize pool.
                </p>

                <dl className="mt-8 grid max-w-2xl grid-cols-2 gap-px overflow-hidden rounded-2xl border bg-border sm:grid-cols-3">
                    <Stat label="Entrants" value={entrants.toLocaleString()} />
                    <Stat label="Entry" value={entryFee != null ? `${formatSol(entryFee)} SOL` : '—'} />
                    <Stat
                        label="Prize pool"
                        value={pot != null ? `${formatSol(pot)} SOL` : '—'}
                        className="col-span-2 sm:col-span-1"
                    />
                </dl>

                <div className="mt-8 flex flex-col gap-3 sm:flex-row sm:items-center">
                    <WalletButton />
                    <span className="inline-flex items-center gap-1.5 text-sm text-muted-foreground">
                        {registrationOpen ? (
                            <>
                                <Sparkles className="size-4" /> Connect a wallet to build your bracket
                            </>
                        ) : (
                            <>
                                <Lock className="size-4" /> Registration is closed — connect to view your bracket
                            </>
                        )}
                    </span>
                </div>

                <div className="mt-4 max-w-xl">
                    <BracketSearch />
                </div>
            </section>

            <section className="mt-16 grid gap-px overflow-hidden rounded-2xl border bg-border sm:grid-cols-3">
                {STEPS.map((step, i) => (
                    <div key={step.title} className="flex flex-col gap-3 bg-card p-6">
                        <span className="font-mono text-sm text-muted-foreground tabular-nums">
                            {String(i + 1).padStart(2, '0')}
                        </span>
                        <h3 className="text-base font-semibold tracking-tight">{step.title}</h3>
                        <p className="text-sm leading-relaxed text-muted-foreground">{step.body}</p>
                    </div>
                ))}
            </section>
        </div>
    );
}

function Stat({ label, value, className }: { label: string; value: string; className?: string }) {
    return (
        <div className={`bg-card p-5 ${className ?? ''}`}>
            <dd className="font-mono text-2xl font-bold tabular-nums leading-none">{value}</dd>
            <dt className="mt-2 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">{label}</dt>
        </div>
    );
}
