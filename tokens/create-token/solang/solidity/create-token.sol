
import "../libraries/spl_token.sol";
import "../libraries/mpl_metadata.sol";

@program_id("8eZPhSaXfHqbcrfskVThtCG68kq8MfVHqmtm6wYf4TLb")
contract create_token {

    // Creating a dataAccount is required by Solang
    // The account is unused in this example
    @payer(payer) // payer account
    constructor() {}

    @mutableSigner(payer) // payer account
    @mutableSigner(mint) // mint account to be created
    @mutableAccount(metadata) // metadata account to be created
    @signer(mintAuthority) // mint authority for the mint account
    @account(rentAddress)
    @account(metadataProgramId)
    function createTokenMint(
        address freezeAuthority, // freeze authority for the mint account
        uint8 decimals, // decimals for the mint account
        string name, // name for the metadata account
        string symbol, // symbol for the metadata account
        string uri // uri for the metadata account
    ) external {
        // Invoke System Program to create a new account for the mint account and,
        // Invoke Token Program to initialize the mint account
        // Set mint authority, freeze authority, and decimals for the mint account
        SplToken.create_mint(
            tx.accounts.payer.key,            // payer account
            tx.accounts.mint.key,            // mint account
            tx.accounts.mintAuthority.key,   // mint authority
            freezeAuthority, // freeze authority
            decimals         // decimals
        );

        // Invoke Metadata Program to create a new account for the metadata account
        MplMetadata.create_metadata_account(
            tx.accounts.metadata.key, // metadata account
            tx.accounts.mint.key,  // mint account
            tx.accounts.mintAuthority.key, // mint authority
            tx.accounts.payer.key, // payer
            tx.accounts.payer.key, // update authority (of the metadata account)
            name, // name
            symbol, // symbol
            uri, // uri (off-chain metadata json)
            tx.accounts.metadataProgramId.key,
            tx.accounts.rentAddress.key
        );
    }
}
