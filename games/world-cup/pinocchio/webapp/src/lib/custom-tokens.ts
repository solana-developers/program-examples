import type { TokenConfig } from '@/config/networks';

const KEY_PREFIX = 'custom-tokens:';

export function readCustomTokens(network: string): TokenConfig[] {
    try {
        const raw = localStorage.getItem(KEY_PREFIX + network);
        if (!raw) return [];
        const parsed = JSON.parse(raw);
        return Array.isArray(parsed) ? (parsed as TokenConfig[]) : [];
    } catch {
        return [];
    }
}

export function addCustomToken(network: string, token: TokenConfig): void {
    const existing = readCustomTokens(network).filter(t => t.mint !== token.mint);
    localStorage.setItem(KEY_PREFIX + network, JSON.stringify([...existing, token]));
}

export function removeCustomToken(network: string, mint: string): void {
    const existing = readCustomTokens(network).filter(t => t.mint !== mint);
    localStorage.setItem(KEY_PREFIX + network, JSON.stringify(existing));
}
