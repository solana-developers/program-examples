import { readFile, writeFile } from 'fs/promises';
import { existsSync } from 'fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const CONFIG_PATH = join(__dirname, '../config.json');

export interface TokenConfig {
    symbol: string;
    name: string;
    mint: string;
    decimals: number;
    type: 'test' | 'mainnet';
    description: string;
}

export interface NetworkConfig {
    programAddress?: string;
    tokens: TokenConfig[];
}

export interface Config {
    networks: Record<string, NetworkConfig>;
}

function migrateOldConfig(raw: Record<string, unknown>): Config {
    if ('networks' in raw && typeof raw.networks === 'object') {
        return raw as unknown as Config;
    }
    const network = (raw.network as string) ?? 'localnet';
    const tokens = (raw.tokens as TokenConfig[]) ?? [];
    return { networks: { [network]: { tokens } } };
}

export async function readConfig(): Promise<Config> {
    if (!existsSync(CONFIG_PATH)) {
        return { networks: {} };
    }
    const content = await readFile(CONFIG_PATH, 'utf-8');
    const raw = JSON.parse(content);
    return migrateOldConfig(raw);
}

export async function writeConfig(config: Config): Promise<void> {
    await writeFile(CONFIG_PATH, JSON.stringify(config, null, 2));
}

export async function addToken(network: string, token: TokenConfig): Promise<void> {
    const config = await readConfig();
    if (!config.networks[network]) {
        config.networks[network] = { tokens: [] };
    }
    const tokens = config.networks[network].tokens;
    const existingIndex = tokens.findIndex(t => t.symbol === token.symbol);
    if (existingIndex >= 0) {
        tokens[existingIndex] = token;
    } else {
        tokens.push(token);
    }
    await writeConfig(config);
}

export async function clearConfig(network?: string): Promise<void> {
    if (network) {
        const config = await readConfig();
        const existing = config.networks[network];
        config.networks[network] = { programAddress: existing?.programAddress, tokens: [] };
        await writeConfig(config);
    } else {
        await writeConfig({ networks: {} });
    }
}

export async function setProgramAddress(network: string, addr: string): Promise<void> {
    const config = await readConfig();
    if (!config.networks[network]) {
        config.networks[network] = { tokens: [] };
    }
    config.networks[network].programAddress = addr;
    await writeConfig(config);
}

export async function getProgramAddress(network: string): Promise<string | null> {
    const config = await readConfig();
    return config.networks[network]?.programAddress ?? null;
}

export async function getUsdcMint(network: string): Promise<string | null> {
    const config = await readConfig();
    const tokens = config.networks[network]?.tokens ?? [];
    const usdc = tokens.find(t => t.symbol === 'USDC');
    return usdc?.mint ?? null;
}
