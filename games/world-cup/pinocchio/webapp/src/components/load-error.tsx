import { AlertTriangle } from 'lucide-react';

import { Button } from '@/components/ui/button';

interface LoadErrorProps {
    onRetry: () => void;
    title?: string;
    body?: string;
}

/** Full-page error state for a failed on-chain read, with a retry. */
export function LoadError({
    onRetry,
    title = 'Couldn’t load this bracket',
    body = 'We hit a network error reaching the cluster. The bracket is safe on-chain — try again.',
}: LoadErrorProps) {
    return (
        <div className="mx-auto flex w-full max-w-xl flex-col items-center gap-4 py-16 text-center">
            <div className="flex size-14 items-center justify-center rounded-2xl bg-destructive/10 text-destructive">
                <AlertTriangle className="size-7" />
            </div>
            <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
            <p className="text-muted-foreground">{body}</p>
            <Button onClick={onRetry} className="mt-2">
                Retry
            </Button>
        </div>
    );
}
