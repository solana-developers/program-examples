import { TEAM_COUNT } from './bracket.js';
import { SLOT_LABELS } from './fifa-2026.js';

/**
 * Display names for the 32 positional team slots (`0..31`), defaulting to the
 * FIFA 2026 seeding: already-qualified teams as country names and the rest as
 * group-position placeholders (`1C`, `2F`, `3ABCDF`, …) — see {@link SLOT_LABELS}.
 *
 * LAUNCH-DAY REPLACEMENT POINT: as the draw fills group positions, this array is
 * the single source the webapp swaps in to flow real names through every display
 * helper, and nothing else needs to change. Callers that want to override without
 * editing this file can pass their own `names` array to the display helpers. The
 * slot order is bracket order and must not change.
 */
export const TEAM_NAMES: readonly string[] = SLOT_LABELS;

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
