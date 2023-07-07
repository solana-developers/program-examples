
import "./system_instruction.sol";
import 'minimum_balance.sol';

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract rent {

    @payer(payer) // The "payer" pays for the data account creation
    constructor(address payer) {}

    function createSystemAccount(address payer, address newAccount, uint64 space) public view {
        // The minimum lamports required for the amount of space allocated to the account
        uint64 lamports = minimum_balance(space);

        SystemInstruction.create_account(
            payer,        // lamports sent from this account (payer)
            newAccount, // lamports sent to this account (account to be created)
            lamports,      // lamport amount (minimum lamports required)
            space,          // space required for the account
            SystemInstruction.systemAddress // program owner (system program)
        );
    }

}
