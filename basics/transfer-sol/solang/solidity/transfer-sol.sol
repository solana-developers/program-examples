
import "../libraries/system_instruction.sol";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract transfer_sol {

    @payer(payer) // payer to create new data account
    constructor() {
        // No data is stored in the account in this example
    }

    // Transfer SOL from one account to another using CPI (Cross Program Invocation) to the System program
    function transferSolWithCpi(address from, address to, uint64 lamports) public {
        // CPI to transfer SOL using "system_instruction" library
        SystemInstruction.transfer(from, to, lamports);
    }

    // Transfer SOL from program owned account to another address by directly modifying the account data lamports
    // This approach only works for accounts owned by the program itself (ex. the dataAccount created in the constructor)
    function transferSolWithProgram(uint64 lamports) public {
        AccountInfo from = tx.accounts[0]; // first account must be an account owned by the program
        AccountInfo to = tx.accounts[1]; // second account must be the intended recipient

        print("From: {:}".format(from.key));
        print("To: {:}".format(to.key));

        from.lamports -= lamports;
        to.lamports += lamports;
    }
}
