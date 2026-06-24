import { type ReactNode } from 'react';
import { GAME_COUNT } from '@solana/world-cup';
import { Info, Medal, Minus, Plus, Trophy } from 'lucide-react';

import { cn } from '@/lib/utils';
import { formatSol } from '@/lib/format';

import { Flag } from './flag';
import { type Pick } from './rounds';
import { displayTeam } from './teams';
import { type BracketBuilder } from './use-bracket-builder';
import { ActionBar, type BracketPending } from './action-bar';

interface SummaryRailProps {
    builder: BracketBuilder;
    entryFee?: bigint | null;
    onReset: () => void;
    onRandom: () => void;
    onYolo: () => void;
    onSubmit: () => void;
    showActions: boolean;
    pending?: BracketPending;
    className?: string;
}

/** Standings panel: progress, the path to the title, third place, and tiebreaker. */
export function SummaryRail({
    builder,
    entryFee,
    onReset,
    onRandom,
    onYolo,
    onSubmit,
    showActions,
    pending,
    className,
}: SummaryRailProps) {
    const { pickCount, tiebreaker, finalists, champion, thirdPlace, isComplete } = builder;
    const pct = Math.round((pickCount / GAME_COUNT) * 100);

    return (
        <div className={cn('flex flex-col gap-5 rounded-2xl border bg-card p-5 shadow-sm', className)}>
            <div className="space-y-2">
                <div className="flex items-baseline justify-between">
                    <h2 className="text-sm font-semibold tracking-tight">Your bracket</h2>
                    <span className="font-mono text-xs text-muted-foreground tabular-nums">
                        {pickCount}/{GAME_COUNT}
                    </span>
                </div>
                <div className="h-1.5 w-full overflow-hidden rounded-full bg-muted">
                    <div
                        className="h-full rounded-full bg-primary transition-[width] duration-300 ease-out"
                        style={{ width: `${pct}%` }}
                    />
                </div>
            </div>

            <div className="space-y-2">
                <SectionLabel>Final</SectionLabel>
                <div className="grid grid-cols-2 gap-2">
                    <SlotChip slot={finalists[0]} />
                    <SlotChip slot={finalists[1]} align="end" />
                </div>
            </div>

            {champion != null ? (
                <div className="hero-entrance flex items-center gap-3 rounded-xl bg-primary p-4 text-primary-foreground">
                    <Trophy className="size-5 shrink-0" />
                    <Flag
                        code={displayTeam(champion).code}
                        title={displayTeam(champion).name}
                        className="h-6 w-9 shrink-0"
                    />
                    <div className="min-w-0">
                        <div className="text-[11px] uppercase tracking-wider opacity-70">Champion</div>
                        <div className="truncate font-bold">{displayTeam(champion).name}</div>
                    </div>
                </div>
            ) : (
                <div className="flex items-center gap-2 rounded-xl border border-dashed p-4 text-sm text-muted-foreground">
                    <Trophy className="size-4 shrink-0" />
                    Crown a champion in the Final
                </div>
            )}

            <div className="flex items-center justify-between gap-2">
                <SectionLabel className="flex items-center gap-1.5">
                    <Medal className="size-3.5" /> Third place
                </SectionLabel>
                <SlotChip slot={thirdPlace} align="end" />
            </div>

            <div className="space-y-2 border-t pt-4">
                <SectionLabel>Tiebreaker — total goals in the Round of 32</SectionLabel>
                <TiebreakerInput value={tiebreaker} onChange={builder.setTiebreaker} />
                <p className="text-[11px] leading-relaxed text-muted-foreground">
                    Closest guess breaks a tie when scores match.
                </p>
            </div>

            <div className="flex items-start gap-2 rounded-lg bg-secondary/50 px-3 py-2 text-[11px] leading-relaxed text-muted-foreground">
                <Info className="mt-px size-3.5 shrink-0" />
                <span>
                    Entry is {entryFee != null ? `${formatSol(entryFee)} SOL` : 'a fixed fee'}. Rent is non-refundable
                    and rolls into the prize pool.
                </span>
            </div>

            {showActions && (
                <ActionBar
                    onReset={onReset}
                    onRandom={onRandom}
                    onYolo={onYolo}
                    onSubmit={onSubmit}
                    canSubmit={isComplete}
                    canClear={pickCount > 0 || tiebreaker != null}
                    pending={pending}
                />
            )}
        </div>
    );
}

function SectionLabel({ children, className }: { children: ReactNode; className?: string }) {
    return (
        <div className={cn('text-[11px] font-semibold uppercase tracking-wider text-muted-foreground', className)}>
            {children}
        </div>
    );
}

function SlotChip({ slot, align = 'start' }: { slot: Pick; align?: 'start' | 'end' }) {
    if (slot == null) {
        return (
            <span className={cn('text-sm italic text-muted-foreground/70', align === 'end' && 'text-right')}>TBD</span>
        );
    }
    const team = displayTeam(slot);
    return (
        <span className={cn('inline-flex min-w-0 items-center gap-1.5 text-sm', align === 'end' && 'justify-end')}>
            <Flag code={team.code} title={team.name} className="h-3.5 w-[21px] shrink-0" />
            <span className="truncate font-medium">{team.name}</span>
        </span>
    );
}

function TiebreakerInput({ value, onChange }: { value: number | null; onChange: (v: number | null) => void }) {
    const step = (delta: number) => onChange((value ?? 0) + delta);
    const atMinimum = (value ?? 0) <= 0;
    return (
        <div className="flex items-stretch gap-2">
            <button
                type="button"
                onClick={() => step(-1)}
                disabled={atMinimum}
                aria-label="Decrease goal guess"
                className="flex size-10 items-center justify-center rounded-lg border bg-background text-muted-foreground transition-colors hover:bg-secondary hover:text-foreground disabled:pointer-events-none disabled:opacity-40"
            >
                <Minus className="size-4" />
            </button>
            <input
                type="number"
                inputMode="numeric"
                aria-label="Total goals scored in the Round of 32"
                min={0}
                placeholder="–"
                value={value ?? ''}
                onChange={e => onChange(e.target.value === '' ? null : Number(e.target.value))}
                className="h-10 w-full min-w-0 rounded-lg border bg-background text-center font-mono text-lg tabular-nums outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
            />
            <button
                type="button"
                onClick={() => step(1)}
                aria-label="Increase goal guess"
                className="flex size-10 items-center justify-center rounded-lg border bg-background text-muted-foreground transition-colors hover:bg-secondary hover:text-foreground"
            >
                <Plus className="size-4" />
            </button>
        </div>
    );
}
