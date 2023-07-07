import 'solana';

// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/instruction/metadata.rs#L449
// Solidity does not support Rust Option<> type, so we need to handle it manually
// Requires creating a struct for each combination of Option<> types
// If bool for Option<> type is false, comment out the corresponding struct field otherwise instruction fails with "invalid account data"
// TODO: figure out better way to handle Option<> types
library MplMetadata {
	address constant metadataProgramId = address"metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
	address constant systemAddress = address"11111111111111111111111111111111";
    address constant rentAddress = address"SysvarRent111111111111111111111111111111111";

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/instruction/metadata.rs#L31
	struct CreateMetadataAccountArgsV3 {
        DataV2 data;
        bool isMutable;
        bool collectionDetailsPresent; // To handle Rust Option<> in Solidity
        // CollectionDetails collectionDetails;
    }

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/state/data.rs#L22
    struct DataV2 {
        string name;
        string symbol;
        string uri;
        uint16 sellerFeeBasisPoints;
        bool creatorsPresent; // To handle Rust Option<> in Solidity
        // Creator[] creators;
        bool collectionPresent; // To handle Rust Option<> in Solidity
        // Collection collection;
        bool usesPresent; // To handle Rust Option<> in Solidity
        // Uses uses;
    }

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/state/metaplex_adapter.rs#L10
    struct Creator {
        address creatorAddress;
        bool verified;
        uint8 share;
    }

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/state/metaplex_adapter.rs#L66
    struct Collection {
        bool verified;
        address key;
    }

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/state/collection.rs#L57
    struct CollectionDetails {
        CollectionDetailsType detailType;
        uint64 size;
    }
    enum CollectionDetailsType {
        V1
    }

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/state/metaplex_adapter.rs#L43
    struct Uses {
        UseMethod useMethod;
        uint64 remaining;
        uint64 total;
    }

	// Reference: https://github.com/metaplex-foundation/metaplex-program-library/blob/master/bubblegum/program/src/state/metaplex_adapter.rs#L35
    enum UseMethod {
        Burn,
        Multiple,
        Single
    }

	function create_metadata_account(
		address metadata,
		address mint,
		address mintAuthority,
		address payer,
		address updateAuthority,
		string name,
		string symbol,
		string uri
	) public view {
        // // Example of how to add a Creator[] array to the DataV2 struct
		// Creator[] memory creators = new Creator[](1);
        // creators[0] = Creator({
        //     creatorAddress: payer,
        //     verified: false,
        //     share: 100
        // });

        DataV2 data = DataV2({
            name: name,
            symbol: symbol,
            uri: uri,
            sellerFeeBasisPoints: 0,
            creatorsPresent: false,
             // creators: creators,
            collectionPresent: false,
            // collection: Collection({
            //     verified: false,
            //     key: address(0)
            // }),
            usesPresent: false
            // uses: Uses({
            //     useMethod: UseMethod.Burn,
            //     remaining: 0,
            //     total: 0
            // })
        });

        CreateMetadataAccountArgsV3 args = CreateMetadataAccountArgsV3({
            data: data,
            isMutable: true,
            collectionDetailsPresent: false
			// collectionDetails: CollectionDetails({
            //     detailType: CollectionDetailsType.V1,
            //     size: 0
            // })
        });

        AccountMeta[7] metas = [
            AccountMeta({pubkey: metadata, is_writable: true, is_signer: false}),
            AccountMeta({pubkey: mint, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: mintAuthority, is_writable: false, is_signer: true}),
            AccountMeta({pubkey: payer, is_writable: true, is_signer: true}),
            AccountMeta({pubkey: updateAuthority, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: systemAddress, is_writable: false, is_signer: false}),
            AccountMeta({pubkey: rentAddress, is_writable: false, is_signer: false})
        ];

        bytes1 discriminator = 33;
        bytes instructionData = abi.encode(discriminator, args);

        metadataProgramId.call{accounts: metas}(instructionData);
    }
}
