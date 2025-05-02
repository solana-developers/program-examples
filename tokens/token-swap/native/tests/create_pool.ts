import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { it, beforeEach, describe } from "mocha";
import { BanksClient, ProgramTestContext, start } from "solana-bankrun";
import { createCreateAmmInstruction, createCreatePoolInstruction } from "./ts/instructions";
import { createMint } from "spl-token-bankrun";
import { Pool } from "./ts/state";
import { expect } from "chai";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { expectRevert } from "./utils";

describe('Create Pool', async () => {
    let programId: PublicKey;
    let context: ProgramTestContext;
    let client: BanksClient;
    let payer: Keypair;
    let ammPda: PublicKey;
    let admin: Keypair;
    let mintA: PublicKey;
    let mintB: PublicKey;
    let poolAccountA: PublicKey;
    let poolAccountB: PublicKey;
    let poolPda: PublicKey;
    let poolAuthorityPda: PublicKey;
    let mintLiquidityPda: PublicKey;
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
        if (!pool) {
            throw new Error(`Pool account not found`);
        }
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
