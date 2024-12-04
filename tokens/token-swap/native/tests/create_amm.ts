import { Keypair } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js"
import { it, beforeEach, describe } from "mocha";
import { start } from "solana-bankrun";
import { createCreateAmmInstruction } from "./ts/instructions";
import { Transaction } from "@solana/web3.js";
import { Amm } from "./ts/state";
import { expect } from 'chai';
import { expectRevert } from "./utils";


describe('Create AMM', async () => {
    let programId, context, client, payer, ammPda, admin;
    beforeEach(async () => {
        programId = PublicKey.unique();
        context = await start([{ name: "token_swap_native", programId }], []);
        client = context.banksClient;
        payer = context.payer;
        ammPda = PublicKey.findProgramAddressSync([Buffer.from('amm'), payer.publicKey.toBuffer()], programId)[0];
        admin = Keypair.generate()
    });

    it('Creation', async () => {
        const fee = 9999
        const ix = createCreateAmmInstruction(ammPda, admin.publicKey, payer.publicKey, programId, fee);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer);

        await client.processTransaction(tx);

        const amm = await client.getAccount(ammPda);
        expect(amm.owner.toBase58()).to.equal(programId.toBase58())
        const ammData = Amm.fromBuffer(Buffer.from(amm.data));
        expect(ammData.admin.toBase58()).to.equal(admin.publicKey.toBase58());
        expect(ammData.fee).to.equal(fee);
    });

    it('Invalid fee', async () => {
        const fee = 10000
        const ix = createCreateAmmInstruction(ammPda, admin.publicKey, payer.publicKey, programId, fee);
        const tx = new Transaction();
        tx.recentBlockhash = context.lastBlockhash;
        tx.add(ix).sign(payer);

        await expectRevert(client.processTransaction(tx));
    });
});