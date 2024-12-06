import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { describe } from "mocha";
import { BanksClient, ProgramTestContext, start } from "solana-bankrun";
import { createCreateAmmInstruction, createCreatePoolInstruction, createDepositLiquidityInstruction } from "./ts/instructions";
import { createMint, createAssociatedTokenAccount, mintTo } from "spl-token-bankrun";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { expect } from "chai";
import { getTokenBalance } from "./utils";

describe('Deposit liquidity', async () => {
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
    let depositorAccountLiquidity: PublicKey;
    let depositorAccountA: PublicKey;
    let depositorAccountB: PublicKey;
    const default_mint_amount = 100 * 10 ** 6;
    const minimum_liquidity = 100; // Matches rust constant
    beforeEach(async () => {
        programId = PublicKey.unique();
        context = await start([{ name: "token_swap_native", programId }], []);
        client = context.banksClient;
        payer = context.payer;
        ammPda = PublicKey.findProgramAddressSync([Buffer.from('amm'), payer.publicKey.toBuffer()], programId)[0];
        admin = Keypair.generate()

        // Create amm
        const fee = 9999
        const ix_amm = createCreateAmmInstruction(ammPda, admin.publicKey, payer.publicKey, programId, fee);
        const tx_amm = new Transaction();
        tx_amm.recentBlockhash = context.lastBlockhash;
        tx_amm.add(ix_amm).sign(payer);
        await client.processTransaction(tx_amm);

        // Initialize the tokens to be used for the pool
        mintA = await createMint(client, payer, admin.publicKey, null, 6,)
        mintB = await createMint(client, payer, admin.publicKey, null, 6,)
        if (mintA.toBuffer().compare(mintB.toBuffer()) >= 0) {
            [mintA, mintB] = [mintB, mintA]
        }

        // Derive accounts
        poolPda = PublicKey.findProgramAddressSync([Buffer.from('pool'), ammPda.toBuffer(), mintA.toBuffer(), mintB.toBuffer()], programId)[0];
        mintLiquidityPda = PublicKey.findProgramAddressSync([Buffer.from('liquidity'), ammPda.toBuffer(), mintA.toBuffer(), mintB.toBuffer()], programId)[0];
        poolAuthorityPda = PublicKey.findProgramAddressSync([Buffer.from('authority'), ammPda.toBuffer(), mintA.toBuffer(), mintB.toBuffer()], programId)[0];
        poolAccountA = await getAssociatedTokenAddressSync(mintA, poolAuthorityPda, true);
        poolAccountB = await getAssociatedTokenAddressSync(mintB, poolAuthorityPda, true);
        depositorAccountLiquidity = await getAssociatedTokenAddressSync(mintLiquidityPda, admin.publicKey);
        depositorAccountA = await getAssociatedTokenAddressSync(mintA, admin.publicKey);
        depositorAccountB = await getAssociatedTokenAddressSync(mintB, admin.publicKey);

        // Create pool
        const ix = createCreatePoolInstruction(ammPda, poolPda, poolAuthorityPda, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, payer.publicKey, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer);
        await client.processTransaction(tx);

        // Mint tokens to depositor account
        await createAssociatedTokenAccount(client, payer, mintLiquidityPda, admin.publicKey);
        await createAssociatedTokenAccount(client, payer, mintA, admin.publicKey);
        await createAssociatedTokenAccount(client, payer, mintB, admin.publicKey);
        await mintTo(client, payer, mintA, depositorAccountA, admin, default_mint_amount);
        await mintTo(client, payer, mintB, depositorAccountB, admin, default_mint_amount);
    });

    it('Deposit equal amounts, twice', async () => {
        const amount_a = 4 * 10 ** 6;
        const amount_b = 4 * 10 ** 6;
        const ix = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a, amount_b, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer, admin);
        await client.processTransaction(tx);

        const depositorLiquidityBalance = await getTokenBalance(client, depositorAccountLiquidity);
        const depositorABalance = await getTokenBalance(client, depositorAccountA);
        const depositorBBalance = await getTokenBalance(client, depositorAccountB);
        const poolABalance = await getTokenBalance(client, poolAccountA);
        const poolBBalance = await getTokenBalance(client, poolAccountB);

        expect(depositorLiquidityBalance).to.equal(amount_a - minimum_liquidity);
        expect(depositorABalance).to.equal(default_mint_amount - amount_a);
        expect(depositorBBalance).to.equal(default_mint_amount - amount_b);
        expect(poolABalance).to.equal(amount_a);
        expect(poolBBalance).to.equal(amount_b);

        // Second transaction
        await context.warpToSlot(BigInt(100));

        const ix2 = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a, amount_b, programId);
        const tx2 = new Transaction();
        tx2.recentBlockhash = context.lastBlockhash;
        tx2.add(ix2).sign(payer, admin);
        await client.processTransaction(tx2);

        const depositorLiquidityBalance2 = await getTokenBalance(client, depositorAccountLiquidity);
        const depositorABalance2 = await getTokenBalance(client, depositorAccountA);
        const depositorBBalance2 = await getTokenBalance(client, depositorAccountB);
        const poolABalance2 = await getTokenBalance(client, poolAccountA);
        const poolBBalance2 = await getTokenBalance(client, poolAccountB);

        expect(depositorLiquidityBalance2).to.equal(depositorLiquidityBalance + amount_a);
        expect(depositorABalance2).to.equal(depositorABalance - amount_a);
        expect(depositorBBalance2).to.equal(depositorBBalance - amount_b);
        expect(poolABalance2).to.equal(poolABalance + amount_a);
        expect(poolBBalance2).to.equal(poolBBalance + amount_b);
    });

    it('Deposit amounts a > b, then a < b', async () => {
        const amount_a = 9 * 10 ** 6;
        const amount_b = 4 * 10 ** 6;

        const ix = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a, amount_b, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer, admin);
        await client.processTransaction(tx);

        const depositorLiquidityBalance = await getTokenBalance(client, depositorAccountLiquidity);
        const depositorABalance = await getTokenBalance(client, depositorAccountA);
        const depositorBBalance = await getTokenBalance(client, depositorAccountB);
        const poolABalance = await getTokenBalance(client, poolAccountA);
        const poolBBalance = await getTokenBalance(client, poolAccountB);

        expect(depositorLiquidityBalance).to.equal(6 * 10 ** 6 - minimum_liquidity);
        expect(depositorABalance).to.equal(default_mint_amount - amount_a);
        expect(depositorBBalance).to.equal(default_mint_amount - amount_b);
        expect(poolABalance).to.equal(amount_a);
        expect(poolBBalance).to.equal(amount_b);

        // Second transaction
        await context.warpToSlot(BigInt(100));

        // Expected behavior is that amount_a gets increased to
        // (27 * 10 ** 6) * (9/4) = 60.75 * 10 ** 6
        // to maintain the ratio established in the above deposit
        const amount_a2 = 18 * 10 ** 6;
        const amount_b2 = 27 * 10 ** 6;

        const ix2 = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a2, amount_b2, programId);
        const tx2 = new Transaction();
        tx2.recentBlockhash = context.lastBlockhash;
        tx2.add(ix2).sign(payer, admin);
        await client.processTransaction(tx2);

        const depositorLiquidityBalance2 = await getTokenBalance(client, depositorAccountLiquidity);
        const depositorABalance2 = await getTokenBalance(client, depositorAccountA);
        const depositorBBalance2 = await getTokenBalance(client, depositorAccountB);
        const poolABalance2 = await getTokenBalance(client, poolAccountA);
        const poolBBalance2 = await getTokenBalance(client, poolAccountB);

        expect(depositorLiquidityBalance2).to.equal(depositorLiquidityBalance + 40.5 * 10 ** 6);
        expect(depositorABalance2).to.equal(depositorABalance - 60.75 * 10 ** 6);
        expect(depositorBBalance2).to.equal(depositorBBalance - amount_b2);
        expect(poolABalance2).to.equal(poolABalance + 60.75 * 10 ** 6);
        expect(poolBBalance2).to.equal(poolBBalance + amount_b2);
    })

    it('Deposit amounts a < b, then a > b', async () => {
        const amount_a = 4 * 10 ** 6;
        const amount_b = 9 * 10 ** 6;

        const ix = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a, amount_b, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer, admin);
        await client.processTransaction(tx);

        const depositorLiquidityBalance = await getTokenBalance(client, depositorAccountLiquidity);
        const depositorABalance = await getTokenBalance(client, depositorAccountA);
        const depositorBBalance = await getTokenBalance(client, depositorAccountB);
        const poolABalance = await getTokenBalance(client, poolAccountA);
        const poolBBalance = await getTokenBalance(client, poolAccountB);

        expect(depositorLiquidityBalance).to.equal(6 * 10 ** 6 - minimum_liquidity);
        expect(depositorABalance).to.equal(default_mint_amount - amount_a);
        expect(depositorBBalance).to.equal(default_mint_amount - amount_b);
        expect(poolABalance).to.equal(amount_a);
        expect(poolBBalance).to.equal(amount_b);

        // Second transaction
        await context.warpToSlot(BigInt(100));

        // Expected behavior is that amount_b gets increased to
        // (27 * 10 ** 6) * (9/4) = 60.75 * 10 ** 6
        // to maintain the ratio established in the above deposit
        const amount_a2 = 27 * 10 ** 6;
        const amount_b2 = 18 * 10 ** 6;

        const ix2 = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a2, amount_b2, programId);
        const tx2 = new Transaction();
        tx2.recentBlockhash = context.lastBlockhash;
        tx2.add(ix2).sign(payer, admin);
        await client.processTransaction(tx2);

        const depositorLiquidityBalance2 = await getTokenBalance(client, depositorAccountLiquidity);
        const depositorABalance2 = await getTokenBalance(client, depositorAccountA);
        const depositorBBalance2 = await getTokenBalance(client, depositorAccountB);
        const poolABalance2 = await getTokenBalance(client, poolAccountA);
        const poolBBalance2 = await getTokenBalance(client, poolAccountB);

        expect(depositorLiquidityBalance2).to.equal(depositorLiquidityBalance + 40.5 * 10 ** 6);
        expect(depositorABalance2).to.equal(depositorABalance - amount_a2);
        expect(depositorBBalance2).to.equal(depositorBBalance - 60.75 * 10 ** 6);
        expect(poolABalance2).to.equal(poolABalance + amount_a2);
        expect(poolBBalance2).to.equal(poolBBalance + 60.75 * 10 ** 6);
    })
});