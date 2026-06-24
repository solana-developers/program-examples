export const BASE58_RE = /^[1-9A-HJ-NP-Za-km-z]{32,44}$/;

export function isValidBase58Address(addr: string): boolean {
    return BASE58_RE.test(addr);
}
