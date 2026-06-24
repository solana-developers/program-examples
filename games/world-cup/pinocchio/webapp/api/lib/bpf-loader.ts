import {
    type Address,
    type Instruction,
    type AccountMeta,
    address,
    getAddressEncoder,
    getProgramDerivedAddress,
} from '@solana/kit';

export const BPF_LOADER_UPGRADEABLE = address('BPFLoaderUpgradeab1e11111111111111111111111');
export const SYSTEM_PROGRAM = address('11111111111111111111111111111111');
export const SYSVAR_RENT = address('SysvarRent111111111111111111111111111111111');
export const SYSVAR_CLOCK = address('SysvarC1ock11111111111111111111111111111111');
export const CHUNK_SIZE = 900;

function u32LE(value: number): Uint8Array {
    const buf = new Uint8Array(4);
    const view = new DataView(buf.buffer);
    view.setUint32(0, value, true);
    return buf;
}

function u64LE(value: number | bigint): Uint8Array {
    const buf = new Uint8Array(8);
    const view = new DataView(buf.buffer);
    view.setBigUint64(0, BigInt(value), true);
    return buf;
}

const AccountRole = { READONLY: 0, WRITABLE: 1, SIGNER: 2, WRITABLE_SIGNER: 3 } as const;

function writable(addr: Address): AccountMeta {
    return { address: addr, role: AccountRole.WRITABLE };
}

function writableSigner(addr: Address): AccountMeta {
    return { address: addr, role: AccountRole.WRITABLE_SIGNER };
}

function signer(addr: Address): AccountMeta {
    return { address: addr, role: AccountRole.SIGNER };
}

function readonly(addr: Address): AccountMeta {
    return { address: addr, role: AccountRole.READONLY };
}

export function buildInitializeBufferIx(buffer: Address, authority: Address): Instruction {
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [writable(buffer), readonly(authority)],
        data: u32LE(0),
    };
}

export function buildWriteIx(buffer: Address, authority: Address, offset: number, chunk: Uint8Array): Instruction {
    const data = new Uint8Array(4 + 4 + 8 + chunk.length);
    data.set(u32LE(1), 0);
    data.set(u32LE(offset), 4);
    data.set(u64LE(chunk.length), 8);
    data.set(chunk, 16);
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [writable(buffer), signer(authority)],
        data,
    };
}

export function buildDeployIx(
    payer: Address,
    programDataPDA: Address,
    program: Address,
    buffer: Address,
    authority: Address,
    maxDataLen: number | bigint,
): Instruction {
    const data = new Uint8Array(4 + 8);
    data.set(u32LE(2), 0);
    data.set(u64LE(maxDataLen), 4);
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            writableSigner(payer),
            writable(programDataPDA),
            writableSigner(program),
            writable(buffer),
            readonly(SYSVAR_RENT),
            readonly(SYSVAR_CLOCK),
            readonly(SYSTEM_PROGRAM),
            signer(authority),
        ],
        data,
    };
}

export function buildUpgradeIx(
    programDataPDA: Address,
    program: Address,
    buffer: Address,
    spill: Address,
    authority: Address,
): Instruction {
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            writable(programDataPDA),
            writable(program),
            writable(buffer),
            writable(spill),
            readonly(SYSVAR_RENT),
            readonly(SYSVAR_CLOCK),
            signer(authority),
        ],
        data: u32LE(3),
    };
}

export function buildSetAuthorityIx(account: Address, currentAuth: Address, newAuth: Address | null): Instruction {
    const accounts = [writable(account), signer(currentAuth)];
    if (newAuth) accounts.push(readonly(newAuth));
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts,
        data: u32LE(4),
    };
}

export function buildCloseBufferIx(buffer: Address, recipient: Address, authority: Address): Instruction {
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [writable(buffer), writable(recipient), signer(authority)],
        data: u32LE(5),
    };
}

export async function deriveProgramDataAddress(programId: Address): Promise<Address> {
    const pubkeyEncoder = getAddressEncoder();
    const [pda] = await getProgramDerivedAddress({
        programAddress: BPF_LOADER_UPGRADEABLE,
        seeds: [pubkeyEncoder.encode(programId)],
    });
    return pda;
}
