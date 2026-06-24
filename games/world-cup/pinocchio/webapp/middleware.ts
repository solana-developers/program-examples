import { shortenAddress } from './src/lib/format';

export const config = { matcher: '/b/:path*' };

/** Escape a value for safe interpolation into an HTML attribute. */
function escapeHtml(value: string): string {
    return value.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

export default async function middleware(request: Request): Promise<Response | undefined> {
    const url = new URL(request.url);
    const owner = url.pathname.split('/')[2] ?? '';

    let html: string;
    try {
        const indexResponse = await fetch(new URL('/index.html', url.origin));
        if (!indexResponse.ok) return undefined;
        html = await indexResponse.text();
    } catch {
        return undefined;
    }

    const image = escapeHtml(`${url.origin}/api/og?b=${encodeURIComponent(owner)}`);
    const title = escapeHtml(`${shortenAddress(owner)}'s World Cup bracket`);
    const description = escapeHtml('See this 2026 World Cup podium — then call your own bracket. #WorldCupSolana');
    const pageUrl = escapeHtml(url.href);
    const meta = [
        '<meta name="twitter:card" content="summary_large_image" />',
        `<meta name="twitter:title" content="${title}" />`,
        `<meta name="twitter:description" content="${description}" />`,
        `<meta name="twitter:image" content="${image}" />`,
        `<meta property="og:title" content="${title}" />`,
        `<meta property="og:description" content="${description}" />`,
        `<meta property="og:image" content="${image}" />`,
        `<meta property="og:url" content="${pageUrl}" />`,
        '<meta property="og:type" content="website" />',
    ].join('');

    return new Response(html.replace('</head>', `${meta}</head>`), {
        headers: {
            'cache-control': 'public, max-age=3600',
            'content-type': 'text/html; charset=utf-8',
        },
    });
}
