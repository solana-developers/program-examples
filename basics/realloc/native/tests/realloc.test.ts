import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
} from '@solana/web3.js';
import {
    createCreateInstruction,
    createKeypairFromFile,
    createReallocateWithoutZeroInitInstruction,
    createReallocateZeroInitInstruction,
    AddressInfo,
    EnhancedAddressInfo,
    WorkInfo,
} from '../ts';



describe("Realloc!", async () => {

    const connection = new Connection(`https://api.devnet.solana.com`, 'confirmed');
    const payer = createKeypairFromFile(require('os').homedir() + '/.config/solana/id.json');
    const program = createKeypairFromFile('./program/target/so/program-keypair.json');

    const testAccount = Keypair.generate();

    it("Create the account with data", async () => {
        console.log(`${testAccount.publicKey}`);
        const ix = createCreateInstruction(
            testAccount.publicKey,
            payer.publicKey, 
            program.publicKey,
            "Jacob",
            123,
            "Main St.",
            "Chicago",
        );
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer, testAccount]
        );
        await printAddressInfo(testAccount.publicKey);
    });
    
    it("Reallocate WITHOUT zero init", async () => {
        const ix = createReallocateWithoutZeroInitInstruction(
            testAccount.publicKey,
            payer.publicKey, 
            program.publicKey,
            "Illinois",
            12345,
        );
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );
        await printEnhancedAddressInfo(testAccount.publicKey);
    });

    it("Reallocate WITH zero init", async () => {
        const ix = createReallocateZeroInitInstruction(
            testAccount.publicKey,
            payer.publicKey, 
            program.publicKey,
            "Pete",
            "Engineer",
            "Solana Labs",
            2,
        );
        await sendAndConfirmTransaction(
            connection, 
            new Transaction().add(ix),
            [payer]
        );
        await printWorkInfo(testAccount.publicKey);
    });

    async function printAddressInfo(pubkey: PublicKey): Promise<void> {
        await delay(2);
        const data = (await connection.getAccountInfo(pubkey))?.data;
        if (data) {
            const addressInfo = AddressInfo.fromBuffer(data);
            console.log("Address info:");
            console.log(`   Name:       ${addressInfo.name}`);
            console.log(`   House Num:  ${addressInfo.house_number}`);
            console.log(`   Street:     ${addressInfo.street}`);
            console.log(`   City:       ${addressInfo.city}`);
        };
    }

    async function printEnhancedAddressInfo(pubkey: PublicKey): Promise<void> {
        await delay(2);
        const data = (await connection.getAccountInfo(pubkey))?.data;
        if (data) {
            const enhancedAddressInfo = EnhancedAddressInfo.fromBuffer(data);
            console.log("Enhanced Address info:");
            console.log(`   Name:       ${enhancedAddressInfo.name}`);
            console.log(`   House Num:  ${enhancedAddressInfo.house_number}`);
            console.log(`   Street:     ${enhancedAddressInfo.street}`);
            console.log(`   City:       ${enhancedAddressInfo.city}`);
            console.log(`   State:      ${enhancedAddressInfo.state}`);
            console.log(`   Zip:        ${enhancedAddressInfo.zip}`);
        };
    }

    async function printWorkInfo(pubkey: PublicKey): Promise<void> {
        await delay(2);
        const data = (await connection.getAccountInfo(pubkey))?.data;
        if (data) {
            const workInfo = WorkInfo.fromBuffer(data);
            console.log("Work info:");
            console.log(`   Name:       ${workInfo.name}`);
            console.log(`   Position:   ${workInfo.position}`);
            console.log(`   Company:    ${workInfo.company}`);
            console.log(`   Years:      ${workInfo.years_employed}`);
        };
    }

    function delay(s: number) {
        return new Promise( resolve => setTimeout(resolve, s * 1000) );
    }
  });
  