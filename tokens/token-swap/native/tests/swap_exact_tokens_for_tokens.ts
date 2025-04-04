import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { describe } from "mocha";
import { BanksClient, ProgramTestContext, start } from "solana-bankrun";
import { createCreateAmmInstruction, createCreatePoolInstruction, createDepositLiquidityInstruction, createSwapExactTokensForTokensInstruction } from "./ts/instructions";
import { createMint, createAssociatedTokenAccount, mintTo } from "spl-token-bankrun";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { getTokenBalance } from "./utils";
import { expect } from "chai";

describe('Swap', async () => {
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
    beforeEach(async () => {
        programId = PublicKey.unique();
        context = await start([{ name: "token_swap_native", programId }], []);
        client = context.banksClient;
        payer = context.payer;
        ammPda = PublicKey.findProgramAddressSync([Buffer.from('amm'), payer.publicKey.toBuffer()], programId)[0];
        admin = Keypair.generate()

        // Create amm
        const fee = 99;
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

    it('Swap from a to b, initial deposit a > b', async () => {
        const amount_a = 9 * 10 ** 6;
        const amount_b = 4 * 10 ** 6;

        const ix = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a, amount_b, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer, admin);
        await client.processTransaction(tx);

        // Second transaction
        await context.warpToSlot(BigInt(100));

        const input = 10 ** 6
        const minOutputAmount = 100
        const ix2 = createSwapExactTokensForTokensInstruction(ammPda, poolPda, poolAuthorityPda, admin.publicKey, mintA, mintB, poolAccountA, poolAccountB, depositorAccountA, depositorAccountB, true, input, minOutputAmount, programId);
        const tx2 = new Transaction();
        tx2.recentBlockhash = context.lastBlockhash;
        tx2.add(ix2).sign(payer, admin);
        await client.processTransaction(tx2);

        const traderABalance = await getTokenBalance(client, depositorAccountA);
        const traderBBalance = await getTokenBalance(client, depositorAccountB);
        const poolABalance = await getTokenBalance(client, poolAccountA);
        const poolBBalance = await getTokenBalance(client, poolAccountB);

        expect(traderABalance).to.equal(default_mint_amount - amount_a - input);
        expect(traderBBalance).to.be.greaterThan(default_mint_amount - amount_b);
        expect(traderBBalance).to.be.lessThan(default_mint_amount - amount_b + input);
        expect(poolABalance).to.equal(amount_a + input);
        expect(poolBBalance).to.be.lessThan(amount_b);
    });

    it('Swap from a to b, initial deposit a < b', async () => {
        const amount_a = 10 * 10 ** 6;
        const amount_b = 30 * 10 ** 6;

        const ix = createDepositLiquidityInstruction(poolPda, poolAuthorityPda, admin.publicKey, mintLiquidityPda, mintA, mintB, poolAccountA, poolAccountB, depositorAccountLiquidity, depositorAccountA, depositorAccountB, amount_a, amount_b, programId);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer, admin);
        await client.processTransaction(tx);

        // Second transaction
        await context.warpToSlot(BigInt(100));

        const input = 10 ** 6
        const minOutputAmount = 100
        const ix2 = createSwapExactTokensForTokensInstruction(ammPda, poolPda, poolAuthorityPda, admin.publicKey, mintA, mintB, poolAccountA, poolAccountB, depositorAccountA, depositorAccountB, true, input, minOutputAmount, programId);
        const tx2 = new Transaction();
        tx2.recentBlockhash = context.lastBlockhash;
        tx2.add(ix2).sign(payer, admin);
        await client.processTransaction(tx2);

        const traderABalance = await getTokenBalance(client, depositorAccountA);
        const traderBBalance = await getTokenBalance(client, depositorAccountB);
        const poolABalance = await getTokenBalance(client, poolAccountA);
        const poolBBalance = await getTokenBalance(client, poolAccountB);

        expect(traderABalance).to.equal(default_mint_amount - amount_a - input);
        expect(traderBBalance).to.be.greaterThan(default_mint_amount - amount_b + input);
        expect(poolABalance).to.equal(amount_a + input);
        expect(poolBBalance).to.be.lessThan(amount_b);
    })
});
