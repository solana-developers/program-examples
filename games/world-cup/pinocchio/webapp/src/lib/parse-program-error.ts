import idl from '@idl';
import { isSolanaError, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM } from '@solana/kit';

type IdlError = { code: number; name: string; message: string };

const errors = (idl.program?.errors ?? []) as IdlError[];
const PROGRAM_ERRORS: Record<number, string> = Object.fromEntries(errors.map(e => [e.code, e.message]));
const PROGRAM_ADDRESS = idl.program?.publicKey ?? '';

/** Resolves a custom error code to a human message. The app only calls the World Cup program. */
function describeCode(code: number, failedProgram = ''): string {
    if (failedProgram && failedProgram !== PROGRAM_ADDRESS) {
        return `Program ${failedProgram} error ${code}`;
    }
    return PROGRAM_ERRORS[code] ?? `World Cup program error ${code}`;
}

/**
 * Finds a custom program-error code by walking the `error` and its `cause` chain
 * (bounded against cycles). kit raises program failures as a structured
 * `SolanaError<INSTRUCTION_ERROR__CUSTOM>`; compute-budget estimation and
 * confirmation nest that error inside their own `cause`.
 */
function findCustomErrorCode(error: Error): number | null {
    let current: unknown = error;
    for (let depth = 0; current != null && depth < 8; depth++) {
        if (isSolanaError(current, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM)) {
            const { code } = current.context;
            if (typeof code === 'number') return code;
        }
        current = current instanceof Error ? (current as { cause?: unknown }).cause : undefined;
    }
    return null;
}

/** Joins an error's message with those of its `cause` chain (bounded against cycles). */
function collectMessages(error: Error): string {
    const messages: string[] = [];
    let current: unknown = error;
    for (let depth = 0; current instanceof Error && depth < 8; depth++) {
        messages.push(current.message);
        current = (current as { cause?: unknown }).cause;
    }
    return messages.join('\n');
}

export function parseProgramError(error: unknown): string {
    if (!(error instanceof Error)) return 'Unknown error';

    // Prefer the structured code kit attaches to a SolanaError — no string parsing.
    const structuredCode = findCustomErrorCode(error);
    if (structuredCode !== null) return describeCode(structuredCode);

    // Fallback: scrape simulation logs / wallet messages across the cause chain.
    const message = collectMessages(error);
    const hexMatch = message.match(/custom program error: 0x([0-9a-fA-F]+)/i);
    const decMatch = message.match(/Custom\((\d+)\)/);
    const code = hexMatch ? parseInt(hexMatch[1], 16) : decMatch ? parseInt(decMatch[1], 10) : null;

    if (code === null) return error.message;

    const failedProgram = message.match(/Program (\w+) failed: custom program error:/)?.[1] ?? '';
    return describeCode(code, failedProgram);
}
