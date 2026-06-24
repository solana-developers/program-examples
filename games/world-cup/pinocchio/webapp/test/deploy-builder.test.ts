import assert from 'node:assert/strict';
import { test } from 'node:test';

import { buildDeployPlan } from '../api/lib/deploy-builder.ts';

test('initial deploy plan does not serialize a program keypair', async () => {
    const programAddress = '11111111111111111111111111111111';
    const plan = await buildDeployPlan(new Uint8Array([1, 2, 3, 4]), programAddress);
    const planRecord = plan as unknown as Record<string, unknown>;

    assert.equal(plan.programAddress, programAddress);
    assert.equal(planRecord.programKeypair, undefined);
    assert.equal(plan.bufferKeypair.length, 64);
    assert.ok(plan.totalChunks > 0);
});
