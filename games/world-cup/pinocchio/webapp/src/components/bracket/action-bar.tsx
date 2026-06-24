import { ArrowRight, Dice5, Loader2, RotateCcw, Shuffle } from 'lucide-react';

import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

/** Which action, if any, has a transaction in flight. */
export type BracketPending = 'submit' | 'yolo' | null;

interface ActionBarProps {
    onReset: () => void;
    onRandom: () => void;
    onYolo: () => void;
    onSubmit: () => void;
    canSubmit: boolean;
    canClear: boolean;
    pending?: BracketPending;
    className?: string;
}

/** Reset / Random / YOLO / Submit controls, used in both the rail and mobile bar. */
export function ActionBar({
    onReset,
    onRandom,
    onYolo,
    onSubmit,
    canSubmit,
    canClear,
    pending = null,
    className,
}: ActionBarProps) {
    const busy = pending !== null;
    return (
        <div className={cn('flex flex-wrap items-center gap-2', className)}>
            <Button variant="ghost" size="sm" onClick={onReset} disabled={!canClear || busy} aria-label="Reset bracket">
                <RotateCcw />
                Reset
            </Button>
            <Button variant="outline" size="sm" onClick={onRandom} disabled={busy} aria-label="Fill a random bracket">
                <Shuffle />
                Random
            </Button>
            <Button variant="secondary" size="sm" onClick={onYolo} disabled={busy} aria-label="Randomize and submit">
                {pending === 'yolo' ? <Loader2 className="animate-spin" /> : <Dice5 />}
                YOLO
            </Button>
            <Button
                size="sm"
                onClick={onSubmit}
                disabled={!canSubmit || busy}
                className="ml-auto min-w-[120px] flex-1 sm:flex-none"
            >
                {pending === 'submit' ? (
                    <>
                        <Loader2 className="animate-spin" />
                        Confirming…
                    </>
                ) : (
                    <>
                        Submit
                        <ArrowRight />
                    </>
                )}
            </Button>
        </div>
    );
}
