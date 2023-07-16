
import "solana";

@program_id("BhDH6TLEnf4dLq9hLn2gLwm5rJdj8Cbdc9ZrsjUpL7kB")
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
    function mint(
        address tree_authority, // authority of the merkle tree
        address leaf_owner, // owner of the new compressed NFT
        address leaf_delegate, // delegate of the new compressed NFT (can be the same as leaf_owner)
        address merkle_tree, // address of the merkle tree
        address payer, // payer
        address tree_delegate, // delegate of the merkle tree
        string uri // uri of the new compressed NFT (metadata)
    ) public {
        print("Minting Compressed NFT");

        // Create a creator array with a single creator
        Creator[] memory creators = new Creator[](1);
        // Set the creator to the payer
        creators[0] = Creator({
            creatorAddress: payer,
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
            AccountMeta({pubkey: tree_authority, is_writable: true, is_signer: false}),
            AccountMeta({pubkey: leaf_owner, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: leaf_delegate, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: merkle_tree, is_writable: true, is_signer: false}),
            AccountMeta({pubkey: payer, is_writable: true, is_signer: true}),
            AccountMeta({pubkey: tree_delegate, is_writable: true, is_signer: true}),
            AccountMeta({pubkey: address"noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV", is_writable: false, is_signer: false}),
            AccountMeta({pubkey: address"cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK", is_writable: false, is_signer: false}),
            AccountMeta({pubkey: address"11111111111111111111111111111111", is_writable: false, is_signer: false})
        ];

        // Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/js/src/generated/instructions/mintV1.ts#L64
        bytes8 discriminator = 0x9162c076b8937668;
        bytes instructionData = abi.encode(discriminator, args);

        // Invoking the Bubblegum program
        address'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY'.call{accounts: metas}(instructionData);
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
