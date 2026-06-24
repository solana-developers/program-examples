import { type ComponentProps } from 'react';

import { Button } from '@/components/ui/button';
import { bracketShareUrl, tweetText, xIntentUrl } from '@/lib/share';

interface ShareButtonProps {
    owner: string;
    picks: ReadonlyArray<number>;
    variant?: ComponentProps<typeof Button>['variant'];
    size?: ComponentProps<typeof Button>['size'];
    label?: string;
    className?: string;
}

function XGlyph() {
    return (
        <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden className="size-4">
            <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24h-6.66l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231 5.45-6.231Zm-1.161 17.52h1.833L7.084 4.126H5.117L17.083 19.77Z" />
        </svg>
    );
}

/** Opens a pre-drafted X post celebrating a bracket, with the public bracket link attached. */
export function ShareButton({
    owner,
    picks,
    variant = 'default',
    size = 'default',
    label = 'Share on X',
    className,
}: ShareButtonProps) {
    function onShare() {
        const url = bracketShareUrl(window.location.origin, owner);
        window.open(xIntentUrl({ text: tweetText(picks), url }), '_blank', 'noopener,noreferrer');
    }

    return (
        <Button type="button" variant={variant} size={size} onClick={onShare} className={className}>
            <XGlyph />
            {label}
        </Button>
    );
}
