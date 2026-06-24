import { ReactQueryProvider } from './react-query-provider';
import { SolanaProvider } from './solana/solana-provider';
import { ErrorBoundary } from 'react-error-boundary';
import React from 'react';

import { SelectedTokenProvider } from '@/hooks/use-selected-token';

function WalletErrorFallback({ error }: { error: unknown }) {
    const errorMessage = error instanceof Error ? error.message : 'An unexpected error occurred';
    if (error instanceof Error) {
        console.error('WalletErrorFallback caught error:', error, error.stack);
    } else {
        console.error('WalletErrorFallback caught non-Error:', error);
    }
    const disconnectAndReload = () => {
        try {
            localStorage.removeItem('connector-kit:v1:account');
            localStorage.removeItem('connector-kit:v1:wallet');
            localStorage.removeItem('connector-kit:v1:wallet-state');
        } catch {
            window.location.reload();
            return;
        }
        window.location.reload();
    };
    return (
        <div className="flex flex-col items-center justify-center min-h-screen gap-4 p-8">
            <h1 className="text-2xl font-bold text-destructive">Wallet Error</h1>
            <p className="text-muted-foreground text-center max-w-md">{errorMessage}</p>
            <div className="flex gap-2">
                <button
                    onClick={disconnectAndReload}
                    className="px-4 py-2 bg-primary text-primary-foreground rounded-full hover:bg-primary/90"
                >
                    Disconnect Wallet & Reload
                </button>
                <button
                    onClick={() => window.location.reload()}
                    className="px-4 py-2 bg-secondary text-secondary-foreground rounded-full hover:bg-secondary/80"
                >
                    Reload Page
                </button>
            </div>
        </div>
    );
}

export function AppProviders({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <ReactQueryProvider>
            <ErrorBoundary FallbackComponent={WalletErrorFallback}>
                <SolanaProvider>
                    <SelectedTokenProvider>{children}</SelectedTokenProvider>
                </SolanaProvider>
            </ErrorBoundary>
        </ReactQueryProvider>
    );
}
