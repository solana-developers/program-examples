
import "./spl_token.sol";
import "./mpl_metadata.sol";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract transfer_tokens {

    @payer(payer)
    constructor(address payer) {}

    function createTokenMint(
        address payer, // payer account
        address mint, // mint account to be created
        address mintAuthority, // mint authority for the mint account
        address freezeAuthority, // freeze authority for the mint account
        address metadata, // metadata account to be created
        uint8 decimals, // decimals for the mint account
        string name, // name for the metadata account
        string symbol, // symbol for the metadata account
        string uri // uri for the metadata account
    ) public view {
        // Invoke System Program to create a new account for the mint account and,
        // Invoke Token Program to initialize the mint account
        // Set mint authority, freeze authority, and decimals for the mint account
        SplToken.create_mint(
            payer,            // payer account
            mint,            // mint account
            mintAuthority,   // mint authority
            freezeAuthority, // freeze authority
            decimals         // decimals
        );

        // Invoke Metadata Program to create a new account for the metadata account
        MplMetadata.create_metadata_account(
            metadata, // metadata account
            mint,  // mint account
            mintAuthority, // mint authority
            payer, // payer
            payer, // update authority (of the metadata account)
            name, // name
            symbol, // symbol
            uri // uri (off-chain metadata json)
        );
    }

    function mintTo(
        address payer, // payer account
        address tokenAccount, // token account to create and receive the minted token
        address mint, // mint account
        address owner, // token account owner
        uint64 amount // amount to mint
    ) public view {
        // Mint token to the token account
        SplToken.mint_to(
            mint, // mint account
            tokenAccount, // token account
            payer, // mint authority
            amount // amount
        );
    }

    // Transfer tokens from one token account to another via Cross Program Invocation to Token Program
    function transferTokens(
        address from, // token account to transfer from
        address to, // token account to transfer to
        uint64 amount // amount to transfer
    ) public view {
        SplToken.TokenAccountData from_data = SplToken.get_token_account_data(from);
        SplToken.transfer(from, to, from_data.owner, amount);
    }
}
