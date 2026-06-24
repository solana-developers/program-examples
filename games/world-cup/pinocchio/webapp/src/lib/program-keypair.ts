import { createKeyPairFromBytes, getAddressFromPublicKey } from '@solana/kit';

export interface ProgramKeypairImport {
    bytes: Uint8Array;
    fileName: string;
    programAddress: string;
}

export async function parseProgramKeypairJson(input: string, fileName = 'keypair.json'): Promise<ProgramKeypairImport> {
    const parsed: unknown = JSON.parse(input);
    if (!Array.isArray(parsed) || parsed.length !== 64) {
        throw new Error('Expected a Solana keypair JSON array with 64 bytes');
    }
    const values = parsed.map(value => {
        if (!Number.isInteger(value) || value < 0 || value > 255) {
            throw new Error('Keypair JSON must contain byte values from 0 to 255');
        }
        return value;
    });
    const bytes = new Uint8Array(values);
    const keypair = await createKeyPairFromBytes(bytes);
    const programAddress = await getAddressFromPublicKey(keypair.publicKey);
    return { bytes, fileName, programAddress: programAddress.toString() };
}
