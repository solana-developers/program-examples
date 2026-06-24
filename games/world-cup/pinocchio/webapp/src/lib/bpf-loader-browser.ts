import {
    type Address,
    type Instruction,
    type TransactionSigner,
    address,
    AccountRole,
    getAddressEncoder,
    getProgramDerivedAddress,
    addSignersToInstruction,
} from '@solana/kit';

export const BPF_LOADER_UPGRADEABLE = address('BPFLoaderUpgradeab1e11111111111111111111111');
export const SYSTEM_PROGRAM = address('11111111111111111111111111111111');
export const SYSVAR_RENT = address('SysvarRent111111111111111111111111111111111');
export const SYSVAR_CLOCK = address('SysvarC1ock11111111111111111111111111111111');
export const CHUNK_SIZE = 900;

function u32LE(value: number): Uint8Array {
    const buf = new Uint8Array(4);
    new DataView(buf.buffer).setUint32(0, value, true);
    return buf;
}

function u64LE(value: number | bigint): Uint8Array {
    const buf = new Uint8Array(8);
    new DataView(buf.buffer).setBigUint64(0, BigInt(value), true);
    return buf;
}

export function buildCreateAccountIx(
    payer: TransactionSigner,
    newAccount: TransactionSigner,
    lamports: bigint,
    space: number,
    owner: Address,
): Instruction {
    const data = new Uint8Array(4 + 8 + 8 + 32);
    const view = new DataView(data.buffer);
    view.setUint32(0, 0, true);
    view.setBigUint64(4, lamports, true);
    view.setBigUint64(12, BigInt(space), true);
    data.set(getAddressEncoder().encode(owner), 20);
    return addSignersToInstruction([payer, newAccount], {
        programAddress: SYSTEM_PROGRAM,
        accounts: [
            { address: payer.address, role: AccountRole.WRITABLE_SIGNER },
            { address: newAccount.address, role: AccountRole.WRITABLE_SIGNER },
        ],
        data,
    });
}

export function buildTransferIx(from: TransactionSigner, to: Address, lamports: bigint): Instruction {
    const data = new Uint8Array(4 + 8);
    const view = new DataView(data.buffer);
    view.setUint32(0, 2, true);
    view.setBigUint64(4, lamports, true);
    return addSignersToInstruction([from], {
        programAddress: SYSTEM_PROGRAM,
        accounts: [
            { address: from.address, role: AccountRole.WRITABLE_SIGNER },
            { address: to, role: AccountRole.WRITABLE },
        ],
        data,
    });
}

export function buildInitializeBufferIx(buffer: Address, authority: Address): Instruction {
    return {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            { address: buffer, role: AccountRole.WRITABLE },
            { address: authority, role: AccountRole.READONLY },
        ],
        data: u32LE(0),
    };
}

export function buildWriteIx(
    buffer: Address,
    authority: TransactionSigner,
    offset: number,
    chunk: Uint8Array,
): Instruction {
    const data = new Uint8Array(4 + 4 + 8 + chunk.length);
    data.set(u32LE(1), 0);
    data.set(u32LE(offset), 4);
    data.set(u64LE(chunk.length), 8);
    data.set(chunk, 16);
    return addSignersToInstruction([authority], {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            { address: buffer, role: AccountRole.WRITABLE },
            { address: authority.address, role: AccountRole.WRITABLE_SIGNER },
        ],
        data,
    });
}

export function buildDeployIx(
    payer: TransactionSigner,
    programDataPDA: Address,
    program: TransactionSigner,
    buffer: Address,
    authority: TransactionSigner,
    maxDataLen: number | bigint,
): Instruction {
    const data = new Uint8Array(4 + 8);
    data.set(u32LE(2), 0);
    data.set(u64LE(maxDataLen), 4);
    return addSignersToInstruction([payer, program, authority], {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            { address: payer.address, role: AccountRole.WRITABLE_SIGNER },
            { address: programDataPDA, role: AccountRole.WRITABLE },
            { address: program.address, role: AccountRole.WRITABLE_SIGNER },
            { address: buffer, role: AccountRole.WRITABLE },
            { address: SYSVAR_RENT, role: AccountRole.READONLY },
            { address: SYSVAR_CLOCK, role: AccountRole.READONLY },
            { address: SYSTEM_PROGRAM, role: AccountRole.READONLY },
            { address: authority.address, role: AccountRole.WRITABLE_SIGNER },
        ],
        data,
    });
}

export function buildUpgradeIx(
    programDataPDA: Address,
    program: Address,
    buffer: Address,
    spill: Address,
    authority: TransactionSigner,
): Instruction {
    return addSignersToInstruction([authority], {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            { address: programDataPDA, role: AccountRole.WRITABLE },
            { address: program, role: AccountRole.WRITABLE },
            { address: buffer, role: AccountRole.WRITABLE },
            { address: spill, role: AccountRole.WRITABLE },
            { address: SYSVAR_RENT, role: AccountRole.READONLY },
            { address: SYSVAR_CLOCK, role: AccountRole.READONLY },
            { address: authority.address, role: AccountRole.WRITABLE_SIGNER },
        ],
        data: u32LE(3),
    });
}

export function buildSetAuthorityIx(
    account: Address,
    currentAuth: TransactionSigner,
    newAuth: Address | null,
): Instruction {
    const accounts: Array<{ address: Address; role: AccountRole }> = [
        { address: account, role: AccountRole.WRITABLE },
        { address: currentAuth.address, role: AccountRole.READONLY_SIGNER },
    ];
    if (newAuth) accounts.push({ address: newAuth, role: AccountRole.READONLY });
    return addSignersToInstruction([currentAuth], {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts,
        data: u32LE(4),
    });
}

export function buildCloseBufferIx(buffer: Address, recipient: Address, authority: TransactionSigner): Instruction {
    return addSignersToInstruction([authority], {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            { address: buffer, role: AccountRole.WRITABLE },
            { address: recipient, role: AccountRole.WRITABLE },
            { address: authority.address, role: AccountRole.WRITABLE_SIGNER },
        ],
        data: u32LE(5),
    });
}

export function buildCloseProgramIx(
    programData: Address,
    recipient: Address,
    authority: TransactionSigner,
    program: Address,
): Instruction {
    return addSignersToInstruction([authority], {
        programAddress: BPF_LOADER_UPGRADEABLE,
        accounts: [
            { address: programData, role: AccountRole.WRITABLE },
            { address: recipient, role: AccountRole.WRITABLE },
            { address: authority.address, role: AccountRole.WRITABLE_SIGNER },
            { address: program, role: AccountRole.WRITABLE },
        ],
        data: u32LE(5),
    });
}

export async function deriveProgramDataAddress(programId: Address | string): Promise<Address> {
    const pubkeyEncoder = getAddressEncoder();
    const addr = typeof programId === 'string' ? address(programId) : programId;
    const [pda] = await getProgramDerivedAddress({
        programAddress: BPF_LOADER_UPGRADEABLE,
        seeds: [pubkeyEncoder.encode(addr)],
    });
    return pda;
}
