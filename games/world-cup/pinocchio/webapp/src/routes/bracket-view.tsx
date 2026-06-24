import { Link, useParams } from 'react-router';
import { isAddress } from '@solana/kit';
import { ArrowLeft, SearchX } from 'lucide-react';

import { BracketDisplay, BracketDisplaySkeleton } from '@/components/bracket/bracket-display';
import { BracketSearch } from '@/components/bracket/bracket-search';
import { LoadError } from '@/components/load-error';
import { Button } from '@/components/ui/button';
import { useBracket } from '@/hooks/use-bracket';
import { useTournament } from '@/hooks/use-tournament';

export function BracketView() {
    const { address } = useParams<{ address: string }>();
    const valid = address != null && isAddress(address);

    const tournament = useTournament();
    const bracket = useBracket(valid ? address : null);

    return (
        <div className="mx-auto w-full max-w-5xl">
            <Button asChild variant="ghost" size="sm" className="mb-4 -ml-2 text-muted-foreground">
                <Link to="/">
                    <ArrowLeft className="size-4" />
                    Home
                </Link>
            </Button>

            {!valid ? (
                <EmptyState title="Invalid address" body="That doesn’t look like a Solana wallet address." />
            ) : tournament.isLoading || bracket.isLoading ? (
                <BracketDisplaySkeleton />
            ) : bracket.data ? (
                <BracketDisplay bracket={bracket.data} oracle={tournament.data?.oracle ?? null} />
            ) : bracket.isError || tournament.isError ? (
                <LoadError
                    onRetry={() => {
                        void bracket.refetch();
                        void tournament.refetch();
                    }}
                />
            ) : (
                <EmptyState title="No bracket found" body="This wallet hasn’t submitted a bracket." />
            )}
        </div>
    );
}

function EmptyState({ title, body }: { title: string; body: string }) {
    return (
        <div className="mx-auto flex w-full max-w-xl flex-col items-center gap-4 py-16 text-center">
            <div className="flex size-14 items-center justify-center rounded-2xl bg-secondary text-muted-foreground">
                <SearchX className="size-7" />
            </div>
            <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
            <p className="text-muted-foreground">{body}</p>
            <BracketSearch className="mt-2" />
        </div>
    );
}
