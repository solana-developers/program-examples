
import "solana";

@program_id("BvgEJTPXfriGPopjJr1nLc4vADXm7A7TqjLFVztpd19Q")
contract compressed_nft {

    @payer(payer) // payer address
    @seed("seed") // hardcoded seed
    constructor(
        @bump bytes1 bump // bump seed for pda address
    ) {
        // Creating a dataAccount for the program, which is required by Solang
        // However, this account is not used in the program
    }

    // Mint a compressed NFT to an existing merkle tree, via a cross-program invocation to the Bubblegum program
    // Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/lib.rs#L922
    // Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/lib.rs#L67
    @mutableAccount(tree_authority) // authority of the merkle tree
    @account(leaf_owner) // owner of the new compressed NFT
    @account(leaf_delegate) // delegate of the new compressed NFT (can be the same as leaf_owner)
    @mutableAccount(merkle_tree)  // address of the merkle tree
    @mutableSigner(payer) // payer
    @mutableSigner(tree_delegate) // delegate of the merkle tree
    @account(noop_address)
    @account(compression_pid)
    @account(bubblegum_pid)
    function mint(
        string uri // uri of the new compressed NFT (metadata)
    ) external {
        print("Minting Compressed NFT");

        // Create a creator array with a single creator
        Creator[] memory creators = new Creator[](1);
        // Set the creator to the payer
        creators[0] = Creator({
            creatorAddress: tx.accounts.payer.key,
            verified: false,
            share: 100
        });

        // Create the metadata args, representing the metadata of the new compressed NFT
        // Solidity does not support optional arguments,
        // So we have to explicitly declare if the optional arguments are present or not
        // If not present, we comment them out, otherwise the transaction will fail with a invalid instruction data error
        MetadataArgs memory args = MetadataArgs({
            name: "RGB",
            symbol: "RGB",
            uri: uri,
            sellerFeeBasisPoints: 0,
            primarySaleHappened: false,
            isMutable: true,
            editionNoncePresent: false,
            // editionNonce: 0,
            tokenStandardPresent: true,
            tokenStandard: TokenStandard.NonFungible,
            collectionPresent: false,
            // collection: Collection({
            //     verified: false,
            //     key: address(0)
            // }),
            usesPresent: false,
            // uses: Uses({
            //     useMethod: UseMethod.Burn,
            //     remaining: 0,
            //     total: 0
            // }),
            tokenProgramVersion: TokenProgramVersion.Original,
            creators: creators
        });

        AccountMeta[9] metas = [
            AccountMeta({pubkey: tx.accounts.tree_authority.key, is_writable: true, is_signer: false}),
            AccountMeta({pubkey: tx.accounts.leaf_owner.key, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: tx.accounts.leaf_delegate.key, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: tx.accounts.merkle_tree.key, is_writable: true, is_signer: false}),
            AccountMeta({pubkey: tx.accounts.payer.key, is_writable: true, is_signer: true}),
            AccountMeta({pubkey: tx.accounts.tree_delegate.key, is_writable: true, is_signer: true}),
            AccountMeta({pubkey: tx.accounts.noop_address.key, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: tx.accounts.compression_pid.key, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: address"11111111111111111111111111111111", is_writable: false, is_signer: false})
        ];

        // Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/js/src/generated/instructions/mintV1.ts#L64
        bytes8 discriminator = 0x9162c076b8937668;
        bytes instructionData = abi.encode(discriminator, args);

        // Invoking the Bubblegum program
        tx.accounts.bubblegum_pid.key.call{accounts: metas}(instructionData);
    }

    // Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/state/metaplex_adapter.rs#L81
    struct MetadataArgs {
        string name;
        string symbol;
        string uri;
        uint16 sellerFeeBasisPoints;
        bool primarySaleHappened;
        bool isMutable;
        bool editionNoncePresent;
        // uint8 editionNonce;
        bool tokenStandardPresent;
        TokenStandard tokenStandard;
        bool collectionPresent;
        // Collection collection;
        bool usesPresent;
        // Uses uses;
        TokenProgramVersion tokenProgramVersion;
        Creator[] creators;
    }

    enum TokenStandard {
        NonFungible,
        FungibleAsset,
        Fungible,
        NonFungibleEdition
    }

    enum TokenProgramVersion {
        Original,
        Token2022
    }

    struct Creator {
        address creatorAddress;
        bool verified;
        uint8 share;
    }

    struct Collection {
        bool verified;
        address key;
    }

    struct Uses {
        UseMethod useMethod;
        uint64 remaining;
        uint64 total;
    }

    enum UseMethod {
        Burn,
        Multiple,
        Single
    }

}
