import {
    appendTransactionMessageInstructions,
    compileTransaction,
    createTransactionMessage,
    getBase64EncodedWireTransaction,
    pipe,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    type Instruction,
    type TransactionSigner,
} from '@solana/kit';

const MAX_TX_BYTES = 1232;

function base64ByteLength(b64: string): number {
    const padding = b64.endsWith('==') ? 2 : b64.endsWith('=') ? 1 : 0;
    return (b64.length * 3) / 4 - padding;
}

function txByteSize(instructions: Instruction[], feePayer: TransactionSigner): number {
    try {
        const tx = pipe(
            createTransactionMessage({ version: 0 }),
            m => setTransactionMessageFeePayerSigner(feePayer, m),
            m =>
                setTransactionMessageLifetimeUsingBlockhash(
                    // eslint-disable-next-line @typescript-eslint/no-explicit-any
                    { blockhash: '11111111111111111111111111111111' as any, lastValidBlockHeight: 0n },
                    m,
                ),
            m => appendTransactionMessageInstructions(instructions, m),
        );
        return base64ByteLength(getBase64EncodedWireTransaction(compileTransaction(tx)));
    } catch {
        return MAX_TX_BYTES + 1;
    }
}

export type InstructionBatch<T> = {
    instructions: Instruction[];
    items: T[];
};

export function packInstructionBatchesWithItems<T extends { instruction: Instruction }>(
    items: T[],
    feePayer: TransactionSigner,
    prefixIxs: Instruction[] = [],
): InstructionBatch<T>[] {
    if (items.length === 0) {
        return prefixIxs.length > 0 ? [{ instructions: prefixIxs, items: [] }] : [];
    }

    const batches: InstructionBatch<T>[] = [];
    let cursor = 0;
    let isFirst = true;

    while (cursor < items.length) {
        const prefix = isFirst ? prefixIxs : [];
        let count = 0;

        for (let i = cursor; i < items.length; i++) {
            const candidate = [...prefix, ...items.slice(cursor, i + 1).map(item => item.instruction)];
            if (txByteSize(candidate, feePayer) > MAX_TX_BYTES) break;
            count = i - cursor + 1;
        }

        const batchItems = items.slice(cursor, cursor + Math.max(count, 1));
        batches.push({
            instructions: [...prefix, ...batchItems.map(item => item.instruction)],
            items: batchItems,
        });
        cursor += batchItems.length;
        isFirst = false;
    }

    return batches;
}

export function packInstructionBatches(
    ixs: Instruction[],
    feePayer: TransactionSigner,
    prefixIxs: Instruction[] = [],
): Instruction[][] {
    return packInstructionBatchesWithItems(
        ixs.map(instruction => ({ instruction })),
        feePayer,
        prefixIxs,
    ).map(batch => batch.instructions);
}
