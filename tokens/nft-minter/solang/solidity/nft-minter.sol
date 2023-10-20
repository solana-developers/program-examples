
import "../libraries/spl_token.sol";
import "../libraries/mpl_metadata.sol";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract nft_minter {

    @payer(payer)
    constructor() {}

    @mutableSigner(payer) // payer account
    @mutableSigner(mint) // mint account to be created
    @mutableAccount(metadata) // metadata account to be created
    @signer(mintAuthority) // mint authority for the mint account
    @account(metadataProgramId)
    @account(rentAddress)
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

    @mutableSigner(payer)
    @mutableAccount(tokenAccount)
    @account(owner)
    @mutableAccount(mint)
    function mintTo() external {
        // Create an associated token account for the owner to receive the minted token
        SplToken.create_associated_token_account(
            tx.accounts.payer.key, // payer account
            tx.accounts.tokenAccount.key, // associated token account address
            tx.accounts.mint.key, // mint account
            tx.accounts.owner.key // owner account
        );

        // Mint 1 token to the associated token account
        SplToken.mint_to(
            tx.accounts.mint.key, // mint account
            tx.accounts.tokenAccount.key, // token account
            tx.accounts.payer.key, // mint authority
            1 // amount
        );

        // Remove mint authority from mint account
        SplToken.remove_mint_authority(
            tx.accounts.mint.key, // mint
            tx.accounts.payer.key // mint authority
        );
    }
}
