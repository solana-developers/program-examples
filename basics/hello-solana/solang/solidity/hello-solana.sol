
@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract hello_solana {
    // The constructor is used to create a new account
    // Here we create a new account that stores no data and only prints messages to the program logs when the constructor is called.
    @payer(payer) // The "payer" pays for the account creation
    constructor(address payer) {
        // We get the program ID by calling 'type(hello_solana).program_id', where "hello_solana" is the name of the contract.
        address programId = type(hello_solana).program_id;

        // Print messages to the program logs
        print("Hello, Solana!");
        print("Our program's Program ID: {:}".format(programId));
    }
}
