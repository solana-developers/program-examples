import type { TokenConfig } from '@/config/networks';

export interface PlanTokenDisplay {
    decimals: number | null;
    mint: string;
    name: string;
    symbol: string;
}

export function resolvePlanTokenDisplay(mint: string, tokens: readonly TokenConfig[] | undefined): PlanTokenDisplay {
    const token = tokens?.find(t => t.mint === mint);
    if (token) return token;

    return {
        decimals: null,
        mint,
        name: 'Unknown token',
        symbol: 'Unknown token',
    };
}

export function formatTokenAmount(amount: bigint, decimals: number): string {
    const divisor = 10n ** BigInt(decimals);
    const whole = amount / divisor;
    const fraction = amount % divisor;
    const wholeText = whole.toLocaleString('en-US');

    if (decimals === 0 || fraction === 0n) return wholeText;

    const fractionText = fraction.toString().padStart(decimals, '0').replace(/0+$/, '');
    return `${wholeText}.${fractionText}`;
}

export function formatPlanTokenAmount(amount: bigint, token: PlanTokenDisplay): string {
    if (token.decimals == null) return `${amount.toLocaleString('en-US')} raw units`;

    return `${formatTokenAmount(amount, token.decimals)} ${token.symbol}`;
}

export function parseTokenAmount(value: string, decimals: number): bigint {
    const trimmed = value.trim();
    if (!/^\d+(\.\d+)?$/.test(trimmed)) {
        throw new Error('Invalid token amount');
    }

    const [whole, fraction = ''] = trimmed.split('.');
    const significantFraction = fraction.replace(/0+$/, '');
    if (significantFraction.length > decimals) {
        throw new Error(`Amount has more than ${decimals} decimal places`);
    }

    const divisor = 10n ** BigInt(decimals);
    const wholeUnits = BigInt(whole) * divisor;
    const fractionUnits = fraction.length === 0 ? 0n : BigInt(fraction.padEnd(decimals, '0') || '0');

    return wholeUnits + fractionUnits;
}
