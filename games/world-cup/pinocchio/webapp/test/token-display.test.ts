import assert from 'node:assert/strict';
import { describe, test } from 'node:test';

import {
    formatPlanTokenAmount,
    formatTokenAmount,
    parseTokenAmount,
    resolvePlanTokenDisplay,
} from '../src/lib/token-display.ts';

describe('token display', () => {
    test('formats known token amounts using configured decimals and symbol', () => {
        const token = resolvePlanTokenDisplay('Mint1111111111111111111111111111111111111', [
            {
                decimals: 6,
                mint: 'Mint1111111111111111111111111111111111111',
                name: 'USD Coin',
                symbol: 'USDC',
            },
        ]);

        assert.equal(token.decimals, 6);
        assert.equal(formatPlanTokenAmount(5_250_000n, token), '5.25 USDC');
    });

    test('does not apply configured token decimals to unknown mints', () => {
        const token = resolvePlanTokenDisplay('Alt11111111111111111111111111111111111111', [
            {
                decimals: 6,
                mint: 'Mint1111111111111111111111111111111111111',
                name: 'USD Coin',
                symbol: 'USDC',
            },
        ]);

        assert.equal(token.decimals, null);
        assert.equal(formatPlanTokenAmount(5_250_000n, token), '5,250,000 raw units');
    });

    test('trims insignificant decimal zeroes', () => {
        assert.equal(formatTokenAmount(1_230_000n, 6), '1.23');
        assert.equal(formatTokenAmount(1_000_000n, 6), '1');
    });

    test('parses token amounts without floating point math', () => {
        assert.equal(parseTokenAmount('1.23', 6), 1_230_000n);
        assert.equal(parseTokenAmount('1.230000', 6), 1_230_000n);
        assert.equal(parseTokenAmount('10', 0), 10n);
        assert.throws(() => parseTokenAmount('1.2345678', 6), /more than 6 decimal places/);
    });
});
