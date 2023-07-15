
import "../libraries/system_instruction.sol";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract pda_rent_payer {

    @payer(payer) // "payer" is the account that pays for creating the account
    @seed("rent_vault") // hardcoded seed
    constructor(@bump bytes1 bump, uint64 fundLamports) {
        // Independently derive the PDA address from the seeds, bump, and programId
        (address pda, bytes1 _bump) = try_find_program_address(["rent_vault"], type(pda_rent_payer).program_id);

        // Verify that the bump passed to the constructor matches the bump derived from the seeds and programId
        // This ensures that only the canonical pda address can be used to create the account (first bump that generates a valid pda address)
        require(bump == _bump, 'INVALID_BUMP');

        // Fund the pda account with additional lamports
        SystemInstruction.transfer(
            tx.accounts.payer.key, // from
            address(this), // to (the address of the account being created)
            fundLamports // amount of lamports to transfer
        );
    }

    function createNewAccount(uint64 lamports) public {
        AccountInfo from = tx.accounts[0]; // first account must be an account owned by the program
        AccountInfo to = tx.accounts[1]; // second account must be the intended recipient

        print("From: {:}".format(from.key));
        print("To: {:}".format(to.key));
        
        // // Not working with Solang 0.3.1
        // from.lamports -= lamports;
        // to.lamports += lamports;
    }
}
