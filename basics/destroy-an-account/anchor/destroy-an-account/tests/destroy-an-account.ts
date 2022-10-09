import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {Keypair, LAMPORTS_PER_SOL} from "@solana/web3.js";
import { DestroyAnAccount } from "../target/types/destroy_an_account";
import {BlockheightBasedTransactionConfirmationStrategy, PublicKey} from "@solana/web3.js";
import assert from "assert";

const PROGRAM_ID = new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

export async function airdrop(program: Program<any>, receiver: PublicKey, amount: number) {
    const sig = await program.provider.connection.requestAirdrop(receiver, amount);
    const blockStats = await program.provider.connection.getLatestBlockhash();
    const strategy: BlockheightBasedTransactionConfirmationStrategy = {
        signature: sig,
        blockhash: blockStats.blockhash,
        lastValidBlockHeight: blockStats.lastValidBlockHeight
    }
    await program.provider.connection.confirmTransaction(strategy, "confirmed");
}

export async function getUserAccount(user: PublicKey): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
        [
            Buffer.from(anchor.utils.bytes.utf8.encode("USER")),
            user.toBuffer()
        ], PROGRAM_ID)
}


describe("destroy-an-account", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.DestroyAnAccount as Program<DestroyAnAccount>;
    const connection = program.provider.connection;
    const user = Keypair.generate();

    it("Airdrop", async () => {
        const balanceBefore = await connection.getBalance(user.publicKey);
        await airdrop(program, user.publicKey, LAMPORTS_PER_SOL);
        const balanceAfter = await connection.getBalance(user.publicKey);
        assert.equal(balanceAfter, balanceBefore + LAMPORTS_PER_SOL);
    });


    it("Create Account", async () => {
        const [userAccountAddress] = await getUserAccount(user.publicKey);
        const userAccountBefore = await program.account.user.fetchNullable(userAccountAddress, "processed");
        assert.equal(userAccountBefore, null);

        await program.methods
            .createUser({
                name: "John Doe"
            })
            .accounts({
                user: user.publicKey,
                userAccount: userAccountAddress
            })
            .signers([user])
            .rpc({commitment: "confirmed"});

        const userAccountAfter = await program.account.user.fetchNullable(userAccountAddress, "processed");
        assert.notEqual(userAccountAfter, null);
        assert.equal(userAccountAfter.name, "John Doe");
        assert.equal(userAccountAfter.user.toBase58(), user.publicKey.toBase58());
    })

    it("Destroy Account", async () => {
        const [userAccountAddress] = await getUserAccount(user.publicKey);
        const userAccountBefore = await program.account.user.fetchNullable(userAccountAddress, "processed");
        assert.notEqual(userAccountBefore, null);

        await program.methods
            .destroyUser()
            .accounts({
                user: user.publicKey,
                userAccount: userAccountAddress
            })
            .signers([user])
            .rpc({commitment: "confirmed"});

        const userAccountAfter = await program.account.user.fetchNullable(userAccountAddress, "processed");
        assert.equal(userAccountAfter, null);
    })
});
