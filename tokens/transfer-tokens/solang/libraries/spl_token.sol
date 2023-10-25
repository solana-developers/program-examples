import 'solana';
import './system_instruction.sol';

library SplToken {
	address constant tokenProgramId = address"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
	address constant associatedTokenProgramId = address"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
	address constant rentAddress = address"SysvarRent111111111111111111111111111111111";
	enum TokenInstruction {
		InitializeMint, // 0
		InitializeAccount, // 1
		InitializeMultisig, // 2
		Transfer, // 3
		Approve, // 4
		Revoke, // 5
		SetAuthority, // 6
		MintTo, // 7
		Burn, // 8
		CloseAccount, // 9
		FreezeAccount, // 10
		ThawAccount, // 11
		TransferChecked, // 12
		ApproveChecked, // 13
		MintToChecked, // 14
		BurnChecked, // 15
		InitializeAccount2, // 16
		SyncNative, // 17
		InitializeAccount3, // 18
		InitializeMultisig2, // 19
		InitializeMint2, // 20
		GetAccountDataSize, // 21
		InitializeImmutableOwner, // 22
		AmountToUiAmount, // 23
		UiAmountToAmount, // 24
		InitializeMintCloseAuthority, // 25
		TransferFeeExtension, // 26
		ConfidentialTransferExtension, // 27
		DefaultAccountStateExtension, // 28
		Reallocate, // 29
		MemoTransferExtension, // 30
		CreateNativeMint // 31
	}

	/// Initialize a new token account.
	///
	/// @param tokenAccount the public key of the token account to initialize
	/// @param mint the public key of the mint account for this new token account
	/// @param owner the public key of the owner of this new token account
	function initialize_account(address tokenAccount, address mint, address owner) internal{
		bytes instr = new bytes(1);

		instr[0] = uint8(TokenInstruction.InitializeAccount);
		AccountMeta[4] metas = [
			AccountMeta({pubkey: tokenAccount, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: mint, is_writable: false, is_signer: false}),
			AccountMeta({pubkey: owner, is_writable: false, is_signer: false}),
			AccountMeta({pubkey: rentAddress, is_writable: false, is_signer: false})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Initialize a new associated token account.
	///
	/// @param payer the public key of the payer to create the associated token account
	/// @param tokenAccount the public key of the token account to initialize
	/// @param mint the public key of the mint account for this new token account
	/// @param owner the public key of the owner of this new token account
    function create_associated_token_account(address payer, address tokenAccount, address mint, address owner) internal {
        AccountMeta[6] metas = [
			AccountMeta({pubkey: payer, is_writable: true, is_signer: true}),
			AccountMeta({pubkey: tokenAccount, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: owner, is_writable: false, is_signer: false}),
			AccountMeta({pubkey: mint, is_writable: false, is_signer: false}),
			AccountMeta({pubkey: SystemInstruction.systemAddress, is_writable: false, is_signer: false}),
			AccountMeta({pubkey: SplToken.tokenProgramId, is_writable: false, is_signer: false})
		];

        bytes instructionData = abi.encode((0));
		associatedTokenProgramId.call{accounts: metas}(instructionData);
    }

	// Initialize mint instruction data
	struct InitializeMintInstruction {
        uint8 instruction;
        uint8 decimals;
        address mintAuthority;
        uint8 freezeAuthorityOption;
        address freezeAuthority;
    }

	/// Initialize a new mint account.
	///
	/// @param mint the public key of the mint account to initialize
	/// @param mintAuthority the public key of the mint authority
	/// @param freezeAuthority the public key of the freeze authority
	/// @param decimals the decimals of the mint
	function initialize_mint(address mint, address mintAuthority, address freezeAuthority, uint8 decimals) internal {
    	InitializeMintInstruction instr = InitializeMintInstruction({
            instruction: 20,
            decimals: decimals,
            mintAuthority: mintAuthority,
            freezeAuthorityOption: 1,
            freezeAuthority: freezeAuthority
        });

		AccountMeta[1] metas = [
			AccountMeta({pubkey: mint, is_writable: true, is_signer: false})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Create and initialize a new mint account in one instruction
	///
	/// @param payer the public key of the account paying to create the mint account
	/// @param mint the public key of the mint account to initialize
	/// @param mintAuthority the public key of the mint authority
	/// @param freezeAuthority the public key of the freeze authority
	/// @param decimals the decimals of the mint
	function create_mint(address payer, address mint, address mintAuthority, address freezeAuthority, uint8 decimals) internal {
		// Invoke System Program to create a new account for the mint account
        // Program owner is set to the Token program
        SystemInstruction.create_account(
            payer,   // lamports sent from this account (payer)
            mint,    // lamports sent to this account (account to be created)
            1461600, // lamport amount (minimum lamports for mint account)
            82,      // space required for the account (mint account)
            SplToken.tokenProgramId // new program owner
        );

		InitializeMintInstruction instr = InitializeMintInstruction({
            instruction: 20,
            decimals: decimals,
            mintAuthority: mintAuthority,
            freezeAuthorityOption: 1,
            freezeAuthority: freezeAuthority
        });

		AccountMeta[1] metas = [
			AccountMeta({pubkey: mint, is_writable: true, is_signer: false})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Mint new tokens. The transaction should be signed by the mint authority keypair
	///
	/// @param mint the account of the mint
	/// @param account the token account where the minted tokens should go
	/// @param authority the public key of the mint authority
	/// @param amount the amount of tokens to mint
	function mint_to(address mint, address account, address authority, uint64 amount) internal {
		bytes instr = new bytes(9);

		instr[0] = uint8(TokenInstruction.MintTo);
		instr.writeUint64LE(amount, 1);

		AccountMeta[3] metas = [
			AccountMeta({pubkey: mint, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: account, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: authority, is_writable: true, is_signer: true})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Transfer @amount token from @from to @to. The transaction should be signed by the owner
	/// keypair of the from account.
	///
	/// @param from the account to transfer tokens from
	/// @param to the account to transfer tokens to
	/// @param owner the publickey of the from account owner keypair
	/// @param amount the amount to transfer
	function transfer(address from, address to, address owner, uint64 amount) internal {
		bytes instr = new bytes(9);

		instr[0] = uint8(TokenInstruction.Transfer);
		instr.writeUint64LE(amount, 1);

		AccountMeta[3] metas = [
			AccountMeta({pubkey: from, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: to, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: owner, is_writable: true, is_signer: true})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Burn @amount tokens in account. This transaction should be signed by the owner.
	///
	/// @param account the acount for which tokens should be burned
	/// @param mint the mint for this token
	/// @param owner the publickey of the account owner keypair
	/// @param amount the amount to transfer
	function burn(address account, address mint, address owner, uint64 amount) internal {
		bytes instr = new bytes(9);

		instr[0] = uint8(TokenInstruction.Burn);
		instr.writeUint64LE(amount, 1);

		AccountMeta[3] metas = [
			AccountMeta({pubkey: account, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: mint, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: owner, is_writable: true, is_signer: true})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Approve an amount to a delegate. This transaction should be signed by the owner
	///
	/// @param account the account for which a delegate should be approved
	/// @param delegate the delegate publickey
	/// @param owner the publickey of the account owner keypair
	/// @param amount the amount to approve
	function approve(address account, address delegate, address owner, uint64 amount) internal {
		bytes instr = new bytes(9);

		instr[0] = uint8(TokenInstruction.Approve);
		instr.writeUint64LE(amount, 1);

		AccountMeta[3] metas = [
			AccountMeta({pubkey: account, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: delegate, is_writable: false, is_signer: false}),
			AccountMeta({pubkey: owner, is_writable: false, is_signer: true})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Revoke a previously approved delegate. This transaction should be signed by the owner. After
	/// this transaction, no delgate is approved for any amount.
	///
	/// @param account the account for which a delegate should be approved
	/// @param owner the publickey of the account owner keypair
	function revoke(address account, address owner) internal {
		bytes instr = new bytes(1);

		instr[0] = uint8(TokenInstruction.Revoke);

		AccountMeta[2] metas = [
			AccountMeta({pubkey: account, is_writable: true, is_signer: false}),
			AccountMeta({pubkey: owner, is_writable: false, is_signer: true})
		];

		tokenProgramId.call{accounts: metas}(instr);
	}

	/// Get the total supply for the mint, i.e. the total amount in circulation
	/// @param mint the mint for this token
	function total_supply(address mint) internal view returns (uint64) {
		AccountInfo account = get_account_info(mint);

		return account.data.readUint64LE(36);
	}

	/// Get the balance for an account.
	///
	/// @param account the account for which we want to know a balance
	function get_balance(address account) internal view returns (uint64) {
		AccountInfo ai = get_account_info(account);

		return ai.data.readUint64LE(64);
	}

	/// Get the account info for an account. This walks the transaction account infos
	/// and find the account info, or the transaction fails.
	///
	/// @param account the account for which we want to have the acount info.
	function get_account_info(address account) internal view returns (AccountInfo) {
		for (uint64 i = 0; i < tx.accounts.length; i++) {
			AccountInfo ai = tx.accounts[i];
			if (ai.key == account) {
				return ai;
			}
		}

		revert("account missing");
	}

	/// This enum represents the state of a token account
	enum AccountState {
		Uninitialized,
		Initialized,
		Frozen
	}

	/// This struct is the return of 'get_token_account_data'
	struct TokenAccountData {
		address mintAccount;
		address owner;
		uint64 balance;
		bool delegate_present;
		address delegate;
		AccountState state;
		bool is_native_present;
		uint64 is_native;
		uint64 delegated_amount;
		bool close_authority_present;
		address close_authority;
	}

	/// Fetch the owner, mint account and balance for an associated token account.
	///
	/// @param tokenAccount The token account
	/// @return struct TokenAccountData
	function get_token_account_data(address tokenAccount) public view returns (TokenAccountData) {
		AccountInfo ai = get_account_info(tokenAccount);

		TokenAccountData data = TokenAccountData(
			{
				mintAccount: ai.data.readAddress(0),
				owner: ai.data.readAddress(32),
			 	balance: ai.data.readUint64LE(64),
				delegate_present: ai.data.readUint32LE(72) > 0,
				delegate: ai.data.readAddress(76),
				state: AccountState(ai.data[108]),
				is_native_present: ai.data.readUint32LE(109) > 0,
				is_native: ai.data.readUint64LE(113),
				delegated_amount: ai.data.readUint64LE(121),
				close_authority_present: ai.data.readUint32LE(129) > 10,
				close_authority: ai.data.readAddress(133)
			}
		);

		return data;
	}

	// This struct is the return of 'get_mint_account_data'
	struct MintAccountData {
		bool authority_present;
		address mint_authority;
		uint64 supply;
		uint8 decimals;
		bool is_initialized;
		bool freeze_authority_present;
		address freeze_authority;
	}

	/// Retrieve the information saved in a mint account
	///
	/// @param mintAccount the account whose information we want to retrive
	/// @return the MintAccountData struct
	function get_mint_account_data(address mintAccount) public view returns (MintAccountData) {
		AccountInfo ai = get_account_info(mintAccount);

		uint32 authority_present = ai.data.readUint32LE(0);
		uint32 freeze_authority_present = ai.data.readUint32LE(46);
		MintAccountData data = MintAccountData( {
			authority_present: authority_present > 0,
			mint_authority: ai.data.readAddress(4),
			supply: ai.data.readUint64LE(36),
			decimals: uint8(ai.data[44]),
			is_initialized: ai.data[45] > 0,
			freeze_authority_present: freeze_authority_present > 0,
			freeze_authority: ai.data.readAddress(50)
		});

		return data;
	}

	// A mint account has an authority, whose type is one of the members of this struct.
	enum AuthorityType {
		MintTokens,
		FreezeAccount,
		AccountOwner,
		CloseAccount
	}

	/// Remove the mint authority from a mint account
	///
	/// @param mintAccount the public key for the mint account
	/// @param mintAuthority the public for the mint authority
	function remove_mint_authority(address mintAccount, address mintAuthority) public {
		AccountMeta[2] metas = [
			AccountMeta({pubkey: mintAccount, is_signer: false, is_writable: true}),
			AccountMeta({pubkey: mintAuthority, is_signer: true, is_writable: false})
		];

		bytes data = new bytes(9);
		data[0] = uint8(TokenInstruction.SetAuthority);
		data[1] = uint8(AuthorityType.MintTokens);
		data[3] = 0;

		tokenProgramId.call{accounts: metas}(data);
	}
}
