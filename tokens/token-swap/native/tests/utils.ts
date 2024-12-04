import { assert } from 'chai';

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