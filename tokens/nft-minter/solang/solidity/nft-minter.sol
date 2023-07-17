
import "../libraries/spl_token.sol";
import "../libraries/mpl_metadata.sol";

@program_id("F1ipperKF9EfD821ZbbYjS319LXYiBmjhzkkf5a26rC")
contract nft_minter {

    @payer(payer)
    constructor() {}

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
    ) public {
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

    function mintTo(address payer, address tokenAccount, address mint, address owner) public {
        // Create an associated token account for the owner to receive the minted token
        SplToken.create_associated_token_account(
            payer, // payer account
            tokenAccount, // associated token account address
            mint, // mint account
            owner // owner account
        );

        // Mint 1 token to the associated token account
        SplToken.mint_to(
            mint, // mint account
            tokenAccount, // token account
            payer, // mint authority
            1 // amount
        );

        // Remove mint authority from mint account
        SplToken.remove_mint_authority(
            mint, // mint
            payer // mint authority
        );
    }
}
