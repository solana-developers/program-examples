import { TEAM_COUNT } from './bracket.js';

/**
 * Display names for the 32 positional team slots (`0..31`), resolved to the real
 * FIFA 2026 knockout-stage qualifiers now that the bracket is final. Each entry's
 * trailing comment keeps the slot's original seeding id (`1C`, `2F`, `3ABCDF`, …
 * from `SLOT_LABELS` in `fifa-2026.ts`) so the mapping back to group positions
 * stays auditable.
 *
 * LAUNCH-DAY REPLACEMENT POINT: this array is the single source the webapp swaps
 * in to flow real names through every display helper, and nothing else needs to
 * change. Callers that want to override without editing this file can pass their
 * own `names` array to the display helpers. The slot order is bracket order and
 * must not change.
 */
export const TEAM_NAMES: readonly string[] = [
    'Germany', // 1E
    'Paraguay', // 3ABCDF
    'France', // 1I
    'Sweden', // 3CDFGH
    'South Africa', // 2A
    'Canada', // 2B
    'Netherlands', // 1F
    'Morocco', // 2C  — slots 0..7
    'Portugal', // 2K
    'Croatia', // 2L
    'Spain', // 1H
    'Austria', // 2J
    'USA', // 1D
    'Bosnia & Herzegovina', // 3BEFIJ
    'Belgium', // 1G
    'Senegal', // 3AEHIJ  — slots 8..15
    'Brazil', // 1C
    'Japan', // 2F
    "Côte d'Ivoire", // 2E
    'Norway', // 2I
    'Mexico', // 1A
    'Ecuador', // 3CEFHI
    'England', // 1L
    'DR Congo', // 3EHIJK  — slots 16..23
    'Argentina', // 1J
    'Cape Verde', // 2H
    'Australia', // 2D
    'Egypt', // 2G
    'Switzerland', // 1B
    'Algeria', // 3EFGIJ
    'Colombia', // 1K
    'Ghana', // 3DEIJL  — slots 24..31
];

/**
 * The display name for a positional team slot. Defaults to {@link TEAM_NAMES};
 * pass `names` to override. Throws `RangeError` when `slot` is outside `0..31`.
 */
export function teamName(slot: number, names: ReadonlyArray<string> = TEAM_NAMES): string {
    if (!Number.isInteger(slot) || slot < 0 || slot >= TEAM_COUNT) {
        throw new RangeError(`team slot ${slot} out of range 0..${TEAM_COUNT}`);
    }
    return names[slot];
}
