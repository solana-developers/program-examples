export const expectRevert = async (promise: Promise<any>) => {
    try {
        await promise;
        throw new Error('Expected a revert');
    } catch {
        return;
    }
};