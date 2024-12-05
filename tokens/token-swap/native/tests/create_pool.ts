import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { it, beforeEach, describe } from "mocha";
import { start } from "solana-bankrun";
import { createCreateAmmInstruction } from "./ts/instructions";
import { createMint } from "spl-token-bankrun";
import { createCreatePoolInstruction } from "./ts/instructions/create_pool";
import { Pool } from "./ts/state";
import { expect } from "chai";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { expectRevert } from "./utils";

describe('Create Pool', async () => {
    let programId, context, client, payer, ammPda, admin;
    let mintA, mintB, poolAccountA, poolAccountB;
    let poolPda, poolAuthorityPda, mintLiquidityPda;
    beforeEach(async () => {
        programId = PublicKey.unique();
        context = await start([{ name: "token_swap_native", programId }], []);
        client = context.banksClient;
        payer = context.payer;
        ammPda = PublicKey.findProgramAddressSync([Buffer.from('amm'), payer.publicKey.toBuffer()], programId)[0];
        admin = Keypair.generate()

        // Create amm
        const fee = 9999
        const ix = createCreateAmmInstruction(ammPda, admin.publicKey, payer.publicKey, programId, fee);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer);

        await client.processTransaction(tx);

        // Initialize tokens to be used for the pool
        mintA = await createMint(client, payer, admin.publicKey, null, 6,)
        mintB = await createMint(client, payer, admin.publicKey, null, 6,)
        if (mintA.toBuffer().compare(mintB.toBuffer()) >= 0) {
            [mintA, mintB] = [mintB, mintA]
        }
        poolPda = PublicKey.findProgramAddressSync([Buffer.from('pool'), ammPda.toBuffer(), mintA.toBuffer(), mintB.toBuffer()], programId)[0];
        mintLiquidityPda = PublicKey.findProgramAddressSync([Buffer.from('liquidity'), ammPda.toBuffer(), mintA.toBuffer(), mintB.toBuffer()], programId)[0];
        poolAuthorityPda = PublicKey.findProgramAddressSync([Buffer.from('authority'), ammPda.toBuffer(), mintA.toBuffer(), mintB.toBuffer()], programId)[0];
        poolAccountA = await getAssociatedTokenAddressSync(mintA, poolAuthorityPda, true);
        poolAccountB = await getAssociatedTokenAddressSync(mintB, poolAuthorityPda, true);
    })

    it('Creation', async () => {
        const ix = createCreatePoolInstruction(ammPda, poolPda, poolAuthorityPda, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, payer.publicKey, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer);

        await client.processTransaction(tx);

        const pool = await client.getAccount(poolPda);
        expect(pool.owner.toBase58()).to.equal(programId.toBase58())
        const poolData = Pool.fromBuffer(Buffer.from(pool.data));
        expect(poolData.amm.toBase58()).to.equal(ammPda.toBase58());
        expect(poolData.mint_a.toBase58()).to.equal(mintA.toBase58());
        expect(poolData.mint_b.toBase58()).to.equal(mintB.toBase58());
    })

    it('Invalid mints', async () => {
        [mintA, mintB] = [mintB, mintA]

        const ix = createCreatePoolInstruction(ammPda, poolPda, poolAuthorityPda, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, payer.publicKey, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer);

        const txPromise = client.processTransaction(tx);
        await expectRevert(txPromise);
    })
});