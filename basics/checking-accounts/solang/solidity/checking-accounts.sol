
import "solana";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract checking_accounts {

    // The dataAccount is unused in this example, but is a required account when using Solang
    @payer(payer) // "payer" is the account that pays to create the dataAccount
    constructor() {}

    @account(accountToChange)
    @mutableSigner(accountToCreate)
    function checkAccounts(address accountToChange, address accountToCreate) external view {
        print("Number of Accounts Provided: {:}".format(tx.accounts.length));

        // Perform checks on the account
        programOwnerCheck(tx.accounts.accountToChange);
        notInitializedCheck(tx.accounts.accountToCreate);
        signerCheck(tx.accounts.accountToCreate);

        // (Create account...) (unimplemented)
        // (Change account...) (unimplemented)
    }

    function programOwnerCheck(AccountInfo account) internal pure {
        print("Progam Owner Check");
        // The owner of this account should be this program
        require(account.owner == type(checking_accounts).program_id, "Account to change does not have the correct program id.");
    }

    function notInitializedCheck(AccountInfo account) internal pure {
        print("Check Account Not Initialized");
        // This account should not be initialized (has no lamports)
        require(account.lamports == 0, "The program expected the account to create to not yet be initialized.");
    }

    function signerCheck(AccountInfo account) internal pure {
        print("Check Account Signed Transaction");
        // This account should be a signer on the transaction
        require(account.is_signer, "Account required to be a signer");
    }
}
