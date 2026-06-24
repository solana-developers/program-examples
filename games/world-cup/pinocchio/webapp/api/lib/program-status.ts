import { getAddressDecoder } from '@solana/kit';

const PROGRAM_ACCOUNT_TAG = 2;
const PROGRAM_DATA_OFFSET = 4;
const PROGRAM_DATA_PUBKEY_LEN = 32;
const PROGRAM_ACCOUNT_MIN_LEN = PROGRAM_DATA_OFFSET + PROGRAM_DATA_PUBKEY_LEN;
const SLOT_OFFSET = 4;
const SLOT_SIZE = 8;
const AUTHORITY_FLAG_OFFSET = 12;
const AUTHORITY_PUBKEY_OFFSET = 13;
const AUTHORITY_PUBKEY_LEN = 32;
const HEADER_SIZE = AUTHORITY_PUBKEY_OFFSET + AUTHORITY_PUBKEY_LEN;

export interface ProgramStatus {
    deployed: boolean;
    upgradeable: boolean;
    upgradeAuthority: string | null;
    lastDeploySlot: number | null;
    lastDeployTime: number | null;
    programDataAddress: string | null;
    dataSize: number | null;
}

interface RpcAccountInfo {
    value: {
        data: [string, string];
        executable: boolean;
        lamports: number;
        owner: string;
        rentEpoch: number;
    } | null;
}

async function rpcCall(rpcUrl: string, method: string, params: unknown[], timeoutMs = 15_000): Promise<unknown> {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), timeoutMs);
    try {
        const res = await fetch(rpcUrl, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }),
            signal: controller.signal,
        });
        const json = (await res.json()) as { result?: unknown; error?: { message: string } };
        if (json.error) throw new Error(json.error.message);
        return json.result;
    } finally {
        clearTimeout(timeout);
    }
}

export async function checkProgramStatus(rpcUrl: string, programAddress: string): Promise<ProgramStatus> {
    const notDeployed: ProgramStatus = {
        deployed: false,
        upgradeable: false,
        upgradeAuthority: null,
        lastDeploySlot: null,
        lastDeployTime: null,
        programDataAddress: null,
        dataSize: null,
    };

    const accountResult = (await rpcCall(rpcUrl, 'getAccountInfo', [
        programAddress,
        { encoding: 'base64' },
    ])) as RpcAccountInfo;

    if (!accountResult.value || !accountResult.value.executable) return notDeployed;

    const data = Buffer.from(accountResult.value.data[0], 'base64');
    if (data.length < PROGRAM_ACCOUNT_MIN_LEN || data[0] !== PROGRAM_ACCOUNT_TAG) return notDeployed;

    const addressDecoder = getAddressDecoder();
    const programDataBytes = data.slice(PROGRAM_DATA_OFFSET, PROGRAM_DATA_OFFSET + PROGRAM_DATA_PUBKEY_LEN);
    const programDataAddress = addressDecoder.decode(programDataBytes);

    const pdResult = (await rpcCall(rpcUrl, 'getAccountInfo', [
        programDataAddress,
        { encoding: 'base64' },
    ])) as RpcAccountInfo;

    if (!pdResult.value) return { ...notDeployed, deployed: true, programDataAddress };

    const pdData = Buffer.from(pdResult.value.data[0], 'base64');

    const slotBytes = pdData.slice(SLOT_OFFSET, SLOT_OFFSET + SLOT_SIZE);
    const lastDeploySlot = Number(slotBytes.reduce((acc, b, i) => acc + BigInt(b) * 256n ** BigInt(i), 0n));

    let lastDeployTime: number | null = null;
    if (lastDeploySlot > 0) {
        try {
            const blockTime = (await rpcCall(rpcUrl, 'getBlockTime', [lastDeploySlot])) as number | null;
            lastDeployTime = blockTime;
        } catch {
            // block time not available for this slot
        }
    }

    const hasAuthority = pdData[AUTHORITY_FLAG_OFFSET] === 1;
    let upgradeAuthority: string | null = null;
    if (hasAuthority && pdData.length >= HEADER_SIZE) {
        upgradeAuthority = addressDecoder.decode(
            pdData.slice(AUTHORITY_PUBKEY_OFFSET, AUTHORITY_PUBKEY_OFFSET + AUTHORITY_PUBKEY_LEN),
        );
    }

    const dataSize = pdData.length - HEADER_SIZE;

    return {
        deployed: true,
        upgradeable: hasAuthority,
        upgradeAuthority,
        lastDeploySlot,
        lastDeployTime,
        programDataAddress,
        dataSize: dataSize > 0 ? dataSize : null,
    };
}
