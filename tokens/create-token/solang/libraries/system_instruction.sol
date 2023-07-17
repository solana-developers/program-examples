// SPDX-License-Identifier: Apache-2.0

// Disclaimer: This library provides a bridge for Solidity to interact with Solana's system instructions. Although it is production ready,
// it has not been audited for security, so use it at your own risk.

import 'solana';

library SystemInstruction {
    address constant systemAddress = address"11111111111111111111111111111111";
    address constant recentBlockHashes = address"SysvarRecentB1ockHashes11111111111111111111";
    address constant rentAddress = address"SysvarRent111111111111111111111111111111111";
    uint64 constant state_size = 80;

    enum Instruction {
        CreateAccount,
        Assign,
        Transfer,
        CreateAccountWithSeed,
        AdvanceNounceAccount,
        WithdrawNonceAccount,
        InitializeNonceAccount,
        AuthorizeNonceAccount,
        Allocate,
        AllocateWithSeed,
        AssignWithSeed,
        TransferWithSeed,
        UpgradeNonceAccount // This is not available on Solana v1.9.15
    }

    /// Create a new account on Solana
    ///
    /// @param from public key for the account from which to transfer lamports to the new account
    /// @param to public key for the account to be created
    /// @param lamports amount of lamports to be transfered to the new account
    /// @param space the size in bytes that is going to be made available for the account
    /// @param owner public key for the program that will own the account being created
    function create_account(address from, address to, uint64 lamports, uint64 space, address owner) internal {
        AccountMeta[2] metas = [
            AccountMeta({pubkey: from, is_signer: true, is_writable: true}),
            AccountMeta({pubkey: to, is_signer: true, is_writable: true})
        ];

        bytes bincode = abi.encode(uint32(Instruction.CreateAccount), lamports, space, owner);

        systemAddress.call{accounts: metas}(bincode);
    }

    /// Create a new account on Solana using a public key derived from a seed
    ///
    /// @param from public key for the account from which to transfer lamports to the new account
    /// @param to the public key for the account to be created. The public key must match create_with_seed(base, seed, owner)
    /// @param base the base address that derived the 'to' address using the seed
    /// @param seed the string utilized to created the 'to' public key
    /// @param lamports amount of lamports to be transfered to the new account
    /// @param space the size in bytes that is going to be made available for the account
    /// @param owner public key for the program that will own the account being created
    function create_account_with_seed(address from, address to, address base, string seed, uint64 lamports, uint64 space, address owner) internal {
        AccountMeta[3] metas = [
            AccountMeta({pubkey: from, is_signer: true, is_writable: true}),
            AccountMeta({pubkey: to, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: base, is_signer: true, is_writable: false})
        ];

        uint32 buffer_size = 92 + seed.length;
        bytes bincode = new bytes(buffer_size);
        bincode.writeUint32LE(uint32(Instruction.CreateAccountWithSeed), 0);
        bincode.writeAddress(base, 4);
        bincode.writeUint64LE(uint64(seed.length), 36);
        bincode.writeString(seed, 44);
        uint32 offset = seed.length + 44;
        bincode.writeUint64LE(lamports, offset);
        offset += 8;
        bincode.writeUint64LE(space, offset);
        offset += 8;
        bincode.writeAddress(owner, offset);

        systemAddress.call{accounts: metas}(bincode);
    }

    /// Assign account to a program (owner)
    ///
    /// @param pubkey the public key for the account whose owner is going to be reassigned
    /// @param owner the public key for the new account owner
    function assign(address pubkey, address owner) internal {
        AccountMeta[1] meta = [
            AccountMeta({pubkey: pubkey, is_signer: true, is_writable: true})
        ];
        bytes bincode = abi.encode(uint32(Instruction.Assign), owner);

        systemAddress.call{accounts: meta}(bincode);
    }

    /// Assign account to a program (owner) based on a seed
    ///
    /// @param addr the public key for the account whose owner is going to be reassigned. The public key must match create_with_seed(base, seed, owner)
    /// @param base the base address that derived the 'addr' key using the seed
    /// @param seed the string utilized to created the 'addr' public key
    /// @param owner the public key for the new program owner
    function assign_with_seed(address addr, address base, string seed, address owner) internal {
        AccountMeta[2] metas = [
            AccountMeta({pubkey: addr, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: base, is_signer: true, is_writable: false})
        ];


        uint32 buffer_size = 76 + seed.length;
        bytes bincode = new bytes(buffer_size);
        bincode.writeUint32LE(uint32(Instruction.AssignWithSeed), 0);
        bincode.writeAddress(base, 4);
        bincode.writeUint64LE(uint64(seed.length), 36);
        bincode.writeString(seed, 44);
        bincode.writeAddress(owner, 44 + seed.length);

        systemAddress.call{accounts: metas}(bincode);
    }

    /// Transfer lamports between accounts
    ///
    /// @param from public key for the funding account
    /// @param to public key for the recipient account
    /// @param lamports amount of lamports to transfer
    function transfer(address from, address to, uint64 lamports) internal {
        AccountMeta[2] metas = [
            AccountMeta({pubkey: from, is_signer: true, is_writable: true}),
            AccountMeta({pubkey: to, is_signer: false, is_writable: true})
        ];

        bytes bincode = abi.encode(uint32(Instruction.Transfer), lamports);

        systemAddress.call{accounts: metas}(bincode);
    }

    /// Transfer lamports from a derived address
    ///
    /// @param from_pubkey The funding account public key. It should match create_with_seed(from_base, seed, from_owner)
    /// @param from_base the base address that derived the 'from_pubkey' key using the seed
    /// @param seed the string utilized to create the 'from_pubkey' public key
    /// @param from_owner owner to use to derive the funding account address
    /// @param to_pubkey the public key for the recipient account
    /// @param lamports amount of lamports to transfer
    function transfer_with_seed(address from_pubkey, address from_base, string seed, address from_owner, address to_pubkey, uint64 lamports) internal {
        AccountMeta[3] metas = [
            AccountMeta({pubkey: from_pubkey, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: from_base, is_signer: true, is_writable: false}),
            AccountMeta({pubkey: to_pubkey, is_signer: false, is_writable: true})
        ];

        uint32 buffer_size = seed.length + 52;
        bytes bincode = new bytes(buffer_size);
        bincode.writeUint32LE(uint32(Instruction.TransferWithSeed), 0);
        bincode.writeUint64LE(lamports, 4);
        bincode.writeUint64LE(seed.length, 12);
        bincode.writeString(seed, 20);
        bincode.writeAddress(from_owner, 20 + seed.length);

        systemAddress.call{accounts: metas}(bincode);
    }

    /// Allocate space in a (possibly new) account without funding
    ///
    /// @param pub_key account for which to allocate space
    /// @param space number of bytes of memory to allocate
    function allocate(address pub_key, uint64 space) internal {
        AccountMeta[1] meta = [
            AccountMeta({pubkey: pub_key, is_signer: true, is_writable: true})
        ];

        bytes bincode = abi.encode(uint32(Instruction.Allocate), space);

        systemAddress.call{accounts: meta}(bincode);
    }

    /// Allocate space for an assign an account at an address derived from a base public key and a seed
    ///
    /// @param addr account for which to allocate space. It should match create_with_seed(base, seed, owner)
    /// @param base the base address that derived the 'addr' key using the seed
    /// @param seed the string utilized to create the 'addr' public key
    /// @param space number of bytes of memory to allocate
    /// @param owner owner to use to derive the 'addr' account address
    function allocate_with_seed(address addr, address base, string seed, uint64 space, address owner) internal {
        AccountMeta[2] metas = [
            AccountMeta({pubkey: addr, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: base, is_signer: true, is_writable: false})
        ];

        bytes bincode = new bytes(seed.length + 84);
        bincode.writeUint32LE(uint32(Instruction.AllocateWithSeed), 0);
        bincode.writeAddress(base, 4);
        bincode.writeUint64LE(seed.length, 36);
        bincode.writeString(seed, 44);
        uint32 offset = 44 + seed.length;
        bincode.writeUint64LE(space, offset);
        offset += 8;
        bincode.writeAddress(owner, offset);

        systemAddress.call{accounts: metas}(bincode);
    }

    /// Create a new nonce account on Solana using a public key derived from a seed
    ///
    /// @param from public key for the account from which to transfer lamports to the new account
    /// @param nonce the public key for the account to be created. The public key must match create_with_seed(base, seed, systemAddress)
    /// @param base the base address that derived the 'nonce' key using the seed
    /// @param seed the string utilized to create the 'addr' public key
    /// @param authority The entity authorized to execute nonce instructions on the account
    /// @param lamports amount of lamports to be transfered to the new account
    function create_nonce_account_with_seed(address from, address nonce, address base, string seed, address authority, uint64 lamports) internal {
        create_account_with_seed(from, nonce, base, seed, lamports, state_size, systemAddress);

        AccountMeta[3] metas = [
            AccountMeta({pubkey: nonce, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: recentBlockHashes, is_signer: false, is_writable: false}),
            AccountMeta({pubkey: rentAddress, is_signer: false, is_writable: false})
        ];

        bytes bincode = abi.encode(uint32(Instruction.InitializeNonceAccount), authority);
        systemAddress.call{accounts: metas}(bincode);
    }

    /// Create a new account on Solana
    ///
    /// @param from public key for the account from which to transfer lamports to the new account
    /// @param nonce the public key for the nonce account to be created
    /// @param authority The entity authorized to execute nonce instructions on the account
    /// @param lamports amount of lamports to be transfered to the new account
    function create_nonce_account(address from, address nonce, address authority, uint64 lamports) internal {
        create_account(from, nonce, lamports, state_size, systemAddress);

        AccountMeta[3] metas = [
            AccountMeta({pubkey: nonce, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: recentBlockHashes, is_signer: false, is_writable: false}),
            AccountMeta({pubkey: rentAddress, is_signer: false, is_writable: false})
        ];

        bytes bincode = abi.encode(uint32(Instruction.InitializeNonceAccount), authority);
        systemAddress.call{accounts: metas}(bincode);
    }

    /// Consumes a stored nonce, replacing it with a successor
    ///
    /// @param nonce_pubkey the public key for the nonce account
    /// @param authorized_pubkey the publick key for the entity authorized to execute instructins on the account
    function advance_nonce_account(address nonce_pubkey, address authorized_pubkey) internal {
        AccountMeta[3] metas = [
            AccountMeta({pubkey: nonce_pubkey, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: recentBlockHashes, is_signer: false, is_writable: false}),
            AccountMeta({pubkey: authorized_pubkey, is_signer: true, is_writable: false})
        ];

        bytes bincode = abi.encode(uint32(Instruction.AdvanceNounceAccount));
        systemAddress.call{accounts: metas}(bincode);
    }

    /// Withdraw funds from a nonce account
    ///
    /// @param nonce_pubkey the public key for the nonce account
    /// @param authorized_pubkey the public key for the entity authorized to execute instructins on the account
    /// @param to_pubkey the recipient account
    /// @param lamports the number of lamports to withdraw
    function withdraw_nonce_account(address nonce_pubkey, address authorized_pubkey, address to_pubkey, uint64 lamports) internal {
        AccountMeta[5] metas = [
            AccountMeta({pubkey: nonce_pubkey, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: to_pubkey, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: recentBlockHashes, is_signer: false, is_writable: false}),
            AccountMeta({pubkey: rentAddress, is_signer: false, is_writable: false}),
            AccountMeta({pubkey: authorized_pubkey, is_signer: true, is_writable: false})
        ];

        bytes bincode = abi.encode(uint32(Instruction.WithdrawNonceAccount), lamports);
        systemAddress.call{accounts: metas}(bincode);
    }

    /// Change the entity authorized to execute nonce instructions on the account
    ///
    /// @param nonce_pubkey the public key for the nonce account
    /// @param authorized_pubkey the public key for the entity authorized to execute instructins on the account
    /// @param new_authority
    function authorize_nonce_account(address nonce_pubkey, address authorized_pubkey, address new_authority) internal {
        AccountMeta[2] metas = [
            AccountMeta({pubkey: nonce_pubkey, is_signer: false, is_writable: true}),
            AccountMeta({pubkey: authorized_pubkey, is_signer: true, is_writable: false})
        ];

        bytes bincode = abi.encode(uint32(Instruction.AuthorizeNonceAccount), new_authority);
        systemAddress.call{accounts: metas}(bincode);
    }

    /// One-time idempotent upgrade of legacy nonce version in order to bump them out of chain domain.
    ///
    /// @param nonce the public key for the nonce account
    // This is not available on Solana v1.9.15
    function upgrade_nonce_account(address nonce) internal {
        AccountMeta[1] meta = [
            AccountMeta({pubkey: nonce, is_signer: false, is_writable: true})
        ];

        bytes bincode = abi.encode(uint32(Instruction.UpgradeNonceAccount));
        systemAddress.call{accounts: meta}(bincode);
    }
}
