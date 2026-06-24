import { TEAM_COUNT, TEAM_NAMES } from '@solana/world-cup';

/** A nation rendered for a positional team slot: a flag code and a name. */
export interface DisplayTeam {
    /** `country-flag-icons` flag id (ISO 3166 code, e.g. `"US"`), or `null` when the
     *  slot is still a group-position placeholder with no nation yet. */
    code: string | null;
    name: string;
}

/**
 * `country-flag-icons` flag id per team name. Names are the client's single source
 * of truth (`TEAM_NAMES`, seeded from the FIFA bracket's `SLOT_LABELS`); this map
 * only resolves a flag. Codes are ISO 3166 alpha-2, with subdivisions underscored
 * (England is `GB_ENG`). Group-position placeholders like `1C`/`3ABCDF` aren't here
 * and render a neutral flag. As the draw fills real nations into the client's
 * `SLOT_LABELS`, add the matching code entry here.
 */
const CODES: Readonly<Record<string, string>> = {
    Argentina: 'AR',
    Australia: 'AU',
    Belgium: 'BE',
    Brazil: 'BR',
    Cameroon: 'CM',
    Canada: 'CA',
    Colombia: 'CO',
    'Costa Rica': 'CR',
    Croatia: 'HR',
    Denmark: 'DK',
    Ecuador: 'EC',
    Egypt: 'EG',
    England: 'GB_ENG',
    France: 'FR',
    Germany: 'DE',
    Ghana: 'GH',
    Iran: 'IR',
    Italy: 'IT',
    Japan: 'JP',
    Mexico: 'MX',
    Morocco: 'MA',
    Netherlands: 'NL',
    Nigeria: 'NG',
    Portugal: 'PT',
    Qatar: 'QA',
    'Saudi Arabia': 'SA',
    Senegal: 'SN',
    'South Africa': 'ZA',
    'South Korea': 'KR',
    Spain: 'ES',
    USA: 'US',
    Uruguay: 'UY',
};

/**
 * The nation for a positional slot: the name comes from the client's seeding
 * (`TEAM_NAMES`), with a flag code resolved from {@link CODES}. Out-of-range ids
 * get a neutral fallback so the UI never renders `undefined`.
 */
export function displayTeam(slot: number): DisplayTeam {
    const name = TEAM_NAMES[slot] ?? (slot >= 0 && slot < TEAM_COUNT ? `Team ${slot + 1}` : 'Unknown');
    return { code: CODES[name] ?? null, name };
}

/**
 * X (Twitter) handle per team name, used to `@`-mention the nation in shared posts.
 * Only confirmed nations are seeded; group-position placeholders have no entry and
 * are skipped. Add entries here as the draw fills real nations into `SLOT_LABELS`.
 */
const HANDLES: Readonly<Record<string, string>> = {
    Germany: '@DFB_Team',
    Mexico: '@miseleccionmx',
    USA: '@USMNT',
};

/** The team's X handle (including the leading `@`), or `null` when none is known. */
export function teamHandle(slot: number): string | null {
    return HANDLES[displayTeam(slot).name] ?? null;
}
