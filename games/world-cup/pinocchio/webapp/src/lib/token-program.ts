import { createSolanaRpc, type Address } from '@solana/kit';

const cache = new Map<string, Address>();

export async function resolveTokenProgram(rpcUrl: string, mint: Address): Promise<Address> {
    const key = `${rpcUrl}:${mint.toString()}`;
    const cached = cache.get(key);
    if (cached) return cached;
    const rpc = createSolanaRpc(rpcUrl);
    const info = await rpc.getAccountInfo(mint, { encoding: 'base64' }).send();
    if (!info.value) throw new Error(`Mint ${mint.toString()} not found`);
    const owner = info.value.owner;
    cache.set(key, owner);
    return owner;
}
