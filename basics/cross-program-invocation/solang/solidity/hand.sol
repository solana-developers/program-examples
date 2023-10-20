
import "solana";

// Interface to the lever program.
@program_id("4wFN9As94uDgcBK9umEi6DNjRLi8gq7jaHwSw3829xq8")
interface leverInterface {
    function switchPower(string name) external;
}

@program_id("9rN5nSQBX1gcbweshWAfRE4Ccv5puJfxUJhqKZ5BEdoP")
contract hand {

    // Creating a data account is required by Solang, but the account is not used in this example.
    // We only interact with the lever program.
    @payer(payer) // payer for the data account
    constructor() {}

    // "Pull the lever" by calling the switchPower instruction on the lever program via a Cross Program Invocation.
    @mutableAccount(leverData)
    function pullLever(string name) external {
        // The account required by the switchPower instruction.
        // This is the data account created by the lever program (not this program), which stores the state of the switch.
        AccountMeta[1] metas = [
            AccountMeta({pubkey: tx.accounts.leverData.key, is_writable: true, is_signer: false})
        ];

        // The data required by the switchPower instruction.
        string instructionData = name;

        // Invoke the switchPower instruction on the lever program.
        leverInterface.switchPower{accounts: metas}(instructionData);
    }
}
