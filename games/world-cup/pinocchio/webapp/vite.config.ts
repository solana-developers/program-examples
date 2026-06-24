import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig, type Plugin } from 'vite';

/** Serves the Vercel Edge functions in `api/` during `vite dev` (Vercel runs them in prod). */
function devApiFunctions(): Plugin {
    return {
        apply: 'serve',
        configureServer(server) {
            server.middlewares.use((req, res, next) => {
                if (req.url !== '/api/og' && !req.url?.startsWith('/api/og?')) return next();
                void (async () => {
                    try {
                        const mod = await server.ssrLoadModule('/api/og.tsx');
                        const handler = mod.default as (request: Request) => Promise<Response>;
                        const request = new Request(new URL(req.url!, `http://${req.headers.host}`));
                        const response = await handler(request);
                        res.statusCode = response.status;
                        response.headers.forEach((value, key) => res.setHeader(key, value));
                        res.end(Buffer.from(await response.arrayBuffer()));
                    } catch (err) {
                        next(err instanceof Error ? err : new Error(String(err)));
                    }
                })();
            });
        },
        name: 'dev-api-functions',
    };
}

export default defineConfig({
    define: {
        'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV ?? 'development'),
    },
    plugins: [react(), devApiFunctions()],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src'),
            '@idl': path.resolve(__dirname, '../idl/world_cup.json'),
        },
        tsconfigPaths: true,
    },
    server: {
        proxy: {
            '/rpc': {
                changeOrigin: true,
                rewrite: path => path.replace(/^\/rpc/, ''),
                target: 'http://localhost:8899',
            },
        },
    },
});
