import * as CountryFlags from 'country-flag-icons/react/3x2';
import { type FC, type SVGProps } from 'react';

import { cn } from '@/lib/utils';

type FlagSvg = FC<SVGProps<SVGSVGElement> & { title?: string }>;

/** All country flag SVGs, keyed by the library's export id (ISO 3166 code, e.g.
 *  `US`, `DE`; subdivisions use an underscore, e.g. `GB_ENG`). */
const FLAGS = CountryFlags as unknown as Record<string, FlagSvg | undefined>;

interface FlagProps {
    /** Library flag id (e.g. `"US"`), or `null` for an unknown/placeholder slot. */
    code: string | null;
    title?: string;
    /** Sizing/spacing classes; pass a 3:2 box (e.g. `h-4 w-6`) to avoid letterboxing. */
    className?: string;
}

/** A crisp SVG country flag, or a neutral placeholder when the slot has no nation yet. */
export function Flag({ code, title, className }: FlagProps) {
    const Svg = code ? FLAGS[code] : undefined;
    if (!Svg) {
        return (
            <span
                aria-hidden
                className={cn('inline-block rounded-[2px] border border-dashed bg-muted/50', className)}
            />
        );
    }
    return <Svg title={title} className={cn('rounded-[2px] ring-1 ring-black/10', className)} />;
}
