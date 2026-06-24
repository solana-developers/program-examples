import { cva } from 'class-variance-authority';
import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { QueryClient } from '@tanstack/react-query';

export const SECONDS_PER_DAY = 86400;

const TIME_DRIFT_ALLOWED_SECS = 120;

export function isExpired(expiryTs: bigint, nowSec?: number): boolean {
    if (expiryTs === 0n) return false;
    const now = nowSec ?? Math.floor(Date.now() / 1000);
    return Number(expiryTs) + TIME_DRIFT_ALLOWED_SECS < now;
}

export function isStillCollectibleSubscription(expiresAtTs: bigint, nowSec?: number): boolean {
    return !isExpired(expiresAtTs, nowSec);
}

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function ellipsify(str = '', len = 4, delimiter = '..') {
    const strLen = str.length;
    const limit = len * 2 + delimiter.length;

    return strLen >= limit ? str.substring(0, len) + delimiter + str.substring(strLen - len, strLen) : str;
}

export function recurringAvailable(
    amountPerPeriod: bigint,
    amountPulledInPeriod: bigint,
    currentPeriodStartTs: bigint | null,
    periodLengthS: bigint,
    blockTime?: number,
): bigint {
    if (currentPeriodStartTs != null && blockTime != null) {
        const periodEnd = Number(currentPeriodStartTs) + Number(periodLengthS);
        if (blockTime >= periodEnd) return amountPerPeriod;
    }
    const pulled = amountPulledInPeriod ?? 0n;
    const remaining = amountPerPeriod - pulled;
    return remaining > 0n ? remaining : 0n;
}

export type InvalidateQueryKeys = readonly (readonly string[])[];

export function invalidateWithDelay(queryClient: QueryClient, queryKeys: InvalidateQueryKeys, delayMs = 500): void {
    queryKeys.forEach(key => queryClient.invalidateQueries({ queryKey: [...key] }));
    setTimeout(() => {
        queryKeys.forEach(key => void queryClient.invalidateQueries({ queryKey: [...key] }));
    }, delayMs);
}

function pad2(n: number): string {
    return n.toString().padStart(2, '0');
}

export function fmtDate(ts: number): string {
    const d = new Date(ts * 1000);
    return `${pad2(d.getDate())}/${pad2(d.getMonth() + 1)}/${d.getFullYear()}`;
}

export function fmtDateTime(ts: number): string {
    const d = new Date(ts * 1000);
    return `${pad2(d.getDate())}/${pad2(d.getMonth() + 1)}/${d.getFullYear()}, ${pad2(d.getHours())}:${pad2(d.getMinutes())}:${pad2(d.getSeconds())}`;
}

export function fmtDateShort(ts: number): string {
    const d = new Date(ts * 1000);
    return `${pad2(d.getDate())}/${pad2(d.getMonth() + 1)}`;
}

export function formatPeriod(hours: bigint, capitalize = false): string {
    const h = Number(hours);
    const map: Record<number, string> = { 24: 'daily', 168: 'weekly', 720: 'monthly', 8760: 'yearly' };
    let result = map[h];
    if (!result) {
        result = h > 24 && h % 24 === 0 ? `every ${h / 24} days` : `every ${h} hours`;
    }
    return capitalize ? result.charAt(0).toUpperCase() + result.slice(1) : result;
}

export function formatPeriodLabel(hours: bigint): string {
    const h = Number(hours);
    if (h === 24) return '1 Day';
    if (h === 168) return '1 Week';
    if (h === 720) return '1 Month';
    if (h === 8760) return '1 Year';
    if (h > 24 && h % 24 === 0) return `${h / 24} Days`;
    return `${h} Hours`;
}

export const SECONDS_PER_HOUR = 3600;

export const buttonVariants = cva(
    "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-full text-sm font-medium transition-all active:scale-[0.98] motion-reduce:transition-none disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 aria-invalid:border-destructive",
    {
        variants: {
            variant: {
                default: 'bg-primary text-primary-foreground shadow-xs hover:bg-primary/90',
                destructive:
                    'bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20',
                outline: 'border border-input bg-background shadow-xs hover:bg-accent hover:text-accent-foreground',
                secondary: 'bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80',
                ghost: 'hover:bg-accent hover:text-accent-foreground',
                link: 'text-primary underline-offset-4 hover:underline',
            },
            size: {
                default: 'h-9 px-4 py-2 has-[>svg]:px-3',
                sm: 'h-8 gap-1.5 px-3 has-[>svg]:px-2.5',
                lg: 'h-10 px-6 has-[>svg]:px-4',
                icon: 'size-9',
            },
        },
        defaultVariants: {
            variant: 'default',
            size: 'default',
        },
    },
);
