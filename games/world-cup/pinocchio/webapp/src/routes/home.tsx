import { useWallet } from '@solana/connector/react';
import { SearchX } from 'lucide-react';

import { BracketDisplay, BracketDisplaySkeleton } from '@/components/bracket/bracket-display';
import { BracketSearch } from '@/components/bracket/bracket-search';
import { Hero } from '@/components/home/hero';
import { LoadError } from '@/components/load-error';
import { useBracket } from '@/hooks/use-bracket';
import { useTournament } from '@/hooks/use-tournament';
import { BracketBuilder } from '@/routes/bracket';

export function Home() {
    const { account } = useWallet();
    const tournament = useTournament();
    const bracket = useBracket(account ?? null);

    if (!account) {
        return <Hero tournament={tournament.data} />;
    }

    if (tournament.isLoading || bracket.isLoading) {
        return <BracketDisplaySkeleton />;
    }

    if (bracket.data) {
        return <BracketDisplay bracket={bracket.data} oracle={tournament.data?.oracle ?? null} />;
    }

    // Don't fall through to the builder on a failed read — a user who already has a
    // bracket would otherwise be shown the form and hit `BracketAlreadyExists` on submit.
    if (bracket.isError || tournament.isError) {
        return (
            <LoadError
                title="Couldn’t load your bracket"
                body="We hit a network error reaching the cluster. Your bracket is safe on-chain — try again."
                onRetry={() => {
                    void bracket.refetch();
                    void tournament.refetch();
                }}
            />
        );
    }

    if (tournament.data?.registrationOpen) {
        return <BracketBuilder />;
    }

    return <NoBracketFound />;
}

function NoBracketFound() {
    return (
        <div className="mx-auto flex w-full max-w-xl flex-col items-center gap-4 py-16 text-center">
            <div className="flex size-14 items-center justify-center rounded-2xl bg-secondary text-muted-foreground">
                <SearchX className="size-7" />
            </div>
            <h1 className="text-2xl font-bold tracking-tight">No bracket found</h1>
            <p className="text-muted-foreground">
                This wallet didn’t enter before registration closed. You can still look up anyone else’s bracket.
            </p>
            <BracketSearch className="mt-2" />
        </div>
    );
}
