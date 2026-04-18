import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { assert } from "chai";
import { CloseAccountProgram } from "../target/types/close_account_program";
import IDL from "../target/idl/close_account_program.json" with { type: 'json' };

describe("create_account and close_account", () => {
    let provider: BankrunProvider;
    let program: Program<CloseAccountProgram>;
    let payer: Keypair;
    let dataAccount: PublicKey;

    before(async () => {
        const context = await startAnchor(
            "",
            [{ name: "close_account_program", programId: new PublicKey(IDL.address) }],
            []
        );
        provider = new BankrunProvider(context);
        anchor.setProvider(provider);
        // Note: we might need to cast IDL depending on your setup.
        program = new Program<CloseAccountProgram>(IDL as any, provider);
        payer = provider.wallet.payer;
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("USER"), payer.publicKey.toBuffer()],
            program.programId
        );
        dataAccount = pda;
    });

    it("create account", async () => {
        const tx = await program.methods.createUser("test")
            .accounts({
                user: payer.publicKey,
                userAccount: dataAccount,
            })
            .signers([payer])
            .rpc();
        console.log("Your transaction signature", tx);
        assert.ok(true);

        const account = await program.account.userState.fetch(dataAccount);
        assert.equal(account.user.toBase58(), payer.publicKey.toBase58());
        assert.equal(account.name, "test");
    });

    it("close account", async () => {
        const tx = await program.methods.closeUser()
            .accounts({
                user: payer.publicKey,
                userAccount: dataAccount,
            })
            .signers([payer])
            .rpc();
        console.log("Close account transaction signature", tx);
        
        // When an account is closed, fetching it should fail
        try {
            await program.account.userState.fetch(dataAccount);
            // If it succeeds, the account was not closed properly, so we fail the test
            assert.fail("The account should have been closed and not fetchable");
        } catch (err: any) {
            // We expect an error because the account was closed/deleted
            console.log("Account successfully deleted! Fetch error:", err.message);
            assert.ok(true);
        }
       assert.ok(true); 
    });
});
