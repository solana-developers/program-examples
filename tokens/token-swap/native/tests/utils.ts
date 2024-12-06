import { assert } from 'chai';


export const getTokenBalance = async (client, tokenAccount) => {
    const account = await client.getAccount(tokenAccount);
    if (!account) {
        throw new Error(`Account ${tokenAccount.toString()} not found`);
    }
    // Token account data has an 8-byte discriminator followed by mint, owner, amount
    // Amount is stored as a u64 at offset 64
    const balance = Buffer.from(account.data).readBigUInt64LE(64);
    return Number(balance);
};

const msg = "Expected revert"
export const expectRevert = async (promise: Promise<any>) => {
    try {
        await promise;
        assert.fail(msg);
    } catch (error) {
        // If this is an assert.fail error, rethrow it
        if (error.message === msg) {
            throw error;
        }
        // Otherwise swallow the error as expected
        return;
    }
};