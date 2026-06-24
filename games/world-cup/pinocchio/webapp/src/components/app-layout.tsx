import React from 'react';
import { Toaster } from './ui/sonner';
import { AppHeader } from './app-header';
import { ClusterChecker } from './cluster/cluster-ui';
import { AccountChecker } from './account/account-ui';

export function AppLayout({ children }: { children: React.ReactNode }) {
    return (
        <div className="min-h-dvh">
            <AppHeader />
            <main className="mx-auto w-full max-w-7xl px-6 pt-24 pb-12">
                <ClusterChecker>
                    <AccountChecker />
                </ClusterChecker>
                {children}
            </main>
            <Toaster />
        </div>
    );
}
