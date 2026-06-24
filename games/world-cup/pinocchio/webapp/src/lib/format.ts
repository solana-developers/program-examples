export function truncateAddress(addr: string, prefixLen = 8, suffixLen = 4): string {
    if (addr.length <= prefixLen + suffixLen + 3) return addr;
    return `${addr.slice(0, prefixLen)}...${addr.slice(-suffixLen)}`;
}

/** A wallet address as `abcd…wxyz` for compact display. */
export function shortenAddress(addr: string): string {
    return addr.length > 10 ? `${addr.slice(0, 4)}…${addr.slice(-4)}` : addr;
}

/** Lamports to a human SOL string (e.g. `0.1`, `1,250`). */
export function formatSol(lamports: bigint | number, fractionDigits = 2): string {
    return (Number(lamports) / 1e9).toLocaleString(undefined, { maximumFractionDigits: fractionDigits });
}
