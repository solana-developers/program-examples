import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from 'react';

import type { TokenConfig } from '@/config/networks';
import { useTokenConfig } from '@/hooks/use-token-config';

const STORAGE_KEY = 'selected-token-mint';

interface SelectedTokenValue {
    selectedMint: string | null;
    selectedToken: TokenConfig | null;
    setSelectedMint: (mint: string) => void;
    tokens: TokenConfig[] | undefined;
}

const SelectedTokenContext = createContext<SelectedTokenValue | null>(null);

export function SelectedTokenProvider({ children }: { children: ReactNode }) {
    const { data: tokens } = useTokenConfig();
    const [override, setOverride] = useState<string | null>(() => {
        try {
            return localStorage.getItem(STORAGE_KEY);
        } catch {
            return null;
        }
    });

    const setSelectedMint = useCallback((mint: string) => {
        try {
            localStorage.setItem(STORAGE_KEY, mint);
        } catch {
            /* empty */
        }
        setOverride(mint);
    }, []);

    const value = useMemo<SelectedTokenValue>(() => {
        const overrideValid = override != null && (tokens?.some(t => t.mint === override) ?? false);
        const selectedMint = (overrideValid ? override : tokens?.[0]?.mint) ?? null;
        const selectedToken = tokens?.find(t => t.mint === selectedMint) ?? null;
        return { selectedMint, selectedToken, setSelectedMint, tokens };
    }, [tokens, override, setSelectedMint]);

    return <SelectedTokenContext.Provider value={value}>{children}</SelectedTokenContext.Provider>;
}

export function useSelectedToken(): SelectedTokenValue {
    const ctx = useContext(SelectedTokenContext);
    if (!ctx) throw new Error('useSelectedToken must be used within SelectedTokenProvider');
    return ctx;
}
