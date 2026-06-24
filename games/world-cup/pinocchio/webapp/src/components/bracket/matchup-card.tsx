import { MATCH_KICKOFFS, matchLabel } from '@solana/world-cup';
import { Check, Lock } from 'lucide-react';

import { cn } from '@/lib/utils';

import { Flag } from './flag';
import { competitorsOf, type Pick } from './rounds';
import { displayTeam } from './teams';

/**
 * Compact kickoff for a game in the viewer's own timezone, e.g. `"Jun 29 ·
 * 1:30 PM PDT"` (or `"29 Jun · 13:30 MESZ"` in a 24-hour locale). The stored
 * kickoff is a UTC instant; `Intl` renders both date and time in the browser's
 * local zone — with a tz abbreviation — so everyone sees their own local time.
 */
const dateFormat = new Intl.DateTimeFormat(undefined, { month: 'short', day: 'numeric' });
const timeFormat = new Intl.DateTimeFormat(undefined, {
    hour: 'numeric',
    minute: '2-digit',
    timeZoneName: 'short',
});

function kickoff(game: number): string {
    const iso = MATCH_KICKOFFS[game];
    if (!iso) return '';
    const at = new Date(iso);
    return `${dateFormat.format(at)} · ${timeFormat.format(at)}`;
}

interface MatchupCardProps {
    game: number;
    picks: readonly Pick[];
    onPick: (game: number, slot: number) => void;
    /** Points a correct pick is worth, shown in the card header. */
    weight?: number;
    className?: string;
}

/** A single matchup: two stacked team rows; tapping a team picks it as the winner. */
export function MatchupCard({ game, picks, onPick, weight, className }: MatchupCardProps) {
    const [a, b] = competitorsOf(game, picks);
    const winner = picks[game];
    const locked = a == null || b == null;
    const when = kickoff(game);

    return (
        <div
            className={cn(
                'overflow-hidden rounded-xl border bg-card shadow-sm transition-colors',
                winner != null ? 'border-foreground/25' : 'hover:border-sand-400',
                className,
            )}
        >
            <div className="flex items-center justify-between border-b bg-secondary/40 px-3 py-1.5 text-[11px] font-medium text-muted-foreground">
                <span className="flex items-baseline gap-2">
                    <span className="uppercase tracking-wider">{matchLabel(game)}</span>
                    {when && <span className="font-normal normal-case opacity-70">{when}</span>}
                </span>
                {weight != null && <span className="font-mono">+{weight}</span>}
            </div>
            <div className="divide-y">
                <TeamRow
                    slot={a}
                    isWinner={winner != null && winner === a}
                    isLoser={winner != null && winner !== a}
                    onPick={() => a != null && onPick(game, a)}
                />
                <TeamRow
                    slot={b}
                    isWinner={winner != null && winner === b}
                    isLoser={winner != null && winner !== b}
                    onPick={() => b != null && onPick(game, b)}
                />
            </div>
            {locked && (
                <div className="flex items-center gap-1.5 border-t bg-secondary/30 px-3 py-1.5 text-[11px] text-muted-foreground">
                    <Lock className="size-3" />
                    Awaiting earlier results
                </div>
            )}
        </div>
    );
}

interface TeamRowProps {
    slot: Pick;
    isWinner: boolean;
    isLoser: boolean;
    onPick: () => void;
}

function TeamRow({ slot, isWinner, isLoser, onPick }: TeamRowProps) {
    if (slot == null) {
        return (
            <div className="flex items-center gap-3 px-3 py-2.5 text-sm text-muted-foreground/70">
                <span className="size-7 shrink-0 rounded-md border border-dashed bg-muted/40" />
                <span className="italic">To be decided</span>
            </div>
        );
    }

    const team = displayTeam(slot);

    return (
        <button
            type="button"
            onClick={onPick}
            aria-pressed={isWinner}
            className={cn(
                'flex w-full items-center gap-3 px-3 py-2.5 text-left text-sm transition-all',
                isWinner ? 'bg-primary font-semibold text-primary-foreground' : 'hover:bg-secondary',
                isLoser && 'opacity-45 hover:opacity-90',
            )}
        >
            <Flag code={team.code} title={team.name} className="h-4 w-6 shrink-0" />
            <span className="flex-1 truncate">{team.name}</span>
            {isWinner ? (
                <Check className="size-4 shrink-0" />
            ) : (
                <span className="shrink-0 rounded bg-muted px-1.5 py-0.5 font-mono text-[11px] tabular-nums text-muted-foreground">
                    {slot + 1}
                </span>
            )}
        </button>
    );
}
