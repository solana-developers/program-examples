const API_BASE_URL = import.meta.env?.VITE_API_URL ?? 'http://localhost:3001';

export class ApiError extends Error {
    status?: number;
    details?: unknown;

    constructor(message: string, status?: number, details?: unknown) {
        super(message);
        this.name = 'ApiError';
        this.status = status;
        this.details = details;
    }
}

export async function apiClient<T>(endpoint: string, options?: RequestInit & { timeout?: number }): Promise<T> {
    const url = `${API_BASE_URL}${endpoint}`;
    const controller = new AbortController();
    const timeoutMs = options?.timeout ?? 15_000;
    const timeout = setTimeout(() => controller.abort(), timeoutMs);

    try {
        const response = await fetch(url, {
            ...options,
            signal: controller.signal,
            headers: {
                'Content-Type': 'application/json',
                ...options?.headers,
            },
        });

        if (!response.ok) {
            const errorData = await response.json().catch(() => ({}));
            const msg = errorData.details
                ? `${errorData.error ?? 'API request failed'}: ${errorData.details}`
                : (errorData.error ?? 'API request failed');
            throw new ApiError(msg, response.status, errorData);
        }

        return await response.json();
    } catch (error) {
        if (error instanceof ApiError) throw error;
        if (error instanceof DOMException && error.name === 'AbortError') {
            throw new ApiError('Request timed out', 408);
        }
        throw new ApiError('Network request failed', undefined, error);
    } finally {
        clearTimeout(timeout);
    }
}

export interface AirdropResponse {
    success: boolean;
    message: string;
    recipient: string;
    amount: number;
    mint?: string;
}

export interface TokenConfig {
    symbol: string;
    name: string;
    mint: string;
    decimals: number;
}

export interface ProgramStatus {
    deployed: boolean;
    upgradeable: boolean;
    upgradeAuthority: string | null;
    lastDeploySlot: number | null;
    lastDeployTime: number | null;
    programDataAddress: string | null;
    dataSize: number | null;
}

export interface DeployPlan {
    bufferKeypair: number[];
    bufferAddress: string;
    chunks: string[];
    totalChunks: number;
    programAddress: string;
    soHash: string;
    soSize: number;
}

export interface NetworkConfigResponse {
    programAddress: string | null;
    tokens: TokenConfig[];
}

export interface FullConfig {
    networks: Record<string, NetworkConfigResponse>;
}

export function clusterIdToNetwork(id: string): string {
    if (id.includes('devnet')) return 'devnet';
    if (id.includes('testnet')) return 'testnet';
    if (id.includes('mainnet')) return 'mainnet';
    return 'localnet';
}

export const api = {
    config: {
        getAll: () => apiClient<FullConfig>('/api/config'),
        getNetworkConfig: (network: string) =>
            apiClient<NetworkConfigResponse>(`/api/tokens?network=${encodeURIComponent(network)}`),
    },
    airdrop: {
        sol: (params: { recipient: string; amount: number }) =>
            apiClient<AirdropResponse>('/api/airdrop/sol', {
                method: 'POST',
                body: JSON.stringify(params),
            }),
        usdc: (params: { recipient: string; amount: number }) =>
            apiClient<AirdropResponse>('/api/airdrop/usdc', {
                method: 'POST',
                body: JSON.stringify(params),
            }),
    },
    setup: {
        startValidator: () =>
            apiClient<{ success: boolean; pid?: number; alreadyRunning?: boolean }>('/api/setup/start-validator', {
                method: 'POST',
            }),
        validatorStatus: () =>
            apiClient<{ validatorRunning: boolean; programDeployed: boolean; programAddress: string }>(
                '/api/setup/validator-status',
            ),
        createMockUsdc: () =>
            apiClient<{ mint: string; alreadyExisted: boolean }>('/api/setup/create-mock-usdc', {
                method: 'POST',
                timeout: 30_000,
            }),
        saveConfig: (params: {
            network: string;
            programAddress?: string;
            tokens: Array<{ symbol: string; mint: string; decimals: number }>;
        }) =>
            apiClient<{ success: boolean }>('/api/setup/save-config', {
                method: 'POST',
                body: JSON.stringify(params),
            }),
    },
    program: {
        status: (programAddress: string, rpcUrl: string) =>
            apiClient<ProgramStatus>(
                `/api/program/status?programAddress=${encodeURIComponent(programAddress)}&rpcUrl=${encodeURIComponent(rpcUrl)}`,
            ),
        prepareDeploy: (params: {
            payerAddress: string;
            programAddress?: string;
            rpcUrl: string;
            isUpgrade: boolean;
        }) =>
            apiClient<DeployPlan>('/api/program/prepare-deploy', {
                method: 'POST',
                body: JSON.stringify(params),
                timeout: 30_000,
            }),
        binaryInfo: () => apiClient<{ hash: string; size: number }>('/api/program/binary-info'),
    },
};
