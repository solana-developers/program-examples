export function extractErrorMessage(e: unknown): string {
    if (!e) return 'Unknown error';
    if (typeof e === 'string') return e;
    if (e instanceof Error) {
        const parts = [e.message];
        const err = e as unknown as Record<string, unknown>;
        if (err.cause instanceof Error) parts.push(`Cause: ${err.cause.message}`);
        else if (err.cause) parts.push(`Cause: ${String(err.cause)}`);
        const context = err.context;
        if (context && typeof context === 'object') {
            const ctx = context as Record<string, unknown>;
            if (ctx.__code) parts.push(`Code: ${ctx.__code}`);
            if (ctx.logs && Array.isArray(ctx.logs))
                parts.push(`Logs: ${(ctx.logs as string[]).slice(-3).join(' | ')}`);
        }
        if (err.logs && Array.isArray(err.logs)) parts.push(`Logs: ${(err.logs as string[]).slice(-3).join(' | ')}`);
        return parts.join(' -- ');
    }
    return String(e);
}
