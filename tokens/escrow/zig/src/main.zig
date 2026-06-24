const sol = @import("solana_program_sdk");
const sol_lib = @import("solana_program_library");
const std = @import("std");
const Rent = sol.rent.Rent;
const PublicKey = sol.public_key.PublicKey;
const Account = sol.account.Account;

export fn entrypoint(input: [*]u8) u64 {
    var context = sol.context.Context.load(input) catch return 1;

    processInstruction(context.program_id, context.accounts[0..context.num_accounts], context.data) catch |err| return @intFromError(err);

    return 0;
}

pub const ProgramError = error{ InvalidIxData, InvalidAcctData, PublicKeyMismatch, Unexpected };

pub const MakeOffer = struct { id: u64, token_a_offered_amount: u64, token_b_wanted_amount: u64 };

pub const Offer = struct { id: u64, maker: PublicKey, token_mint_a: PublicKey, token_mint_b: PublicKey, token_b_wanted_amount: u64, bump: u8 };

pub const InstructionType = enum(u8) { make, take };

pub fn processInstruction(program_id: *PublicKey, accounts: []Account, data: []const u8) ProgramError!void {
    const instruction_type: *const InstructionType = @ptrCast(data);

    switch (instruction_type.*) {
        InstructionType.make => {
            const make_data: *align(1) const MakeOffer = @ptrCast(data[1..]);

            try make_offer(program_id, accounts, make_data.*);
        },
        InstructionType.take => {
            try take_offer(program_id, accounts);
        },
    }
}

pub fn make_offer(program_id: *PublicKey, accounts: []Account, data: MakeOffer) ProgramError!void {
    if (!(accounts.len == 10)) return ProgramError.InvalidAcctData;

    const offer_info = accounts[0];
    const token_mint_a = accounts[1];
    const token_mint_b = accounts[2];
    const maker_token_account_a = accounts[3];
    const vault = accounts[4];
    const maker = accounts[5];
    const payer = accounts[6];
    const token_program = accounts[7];
    const system_program = accounts[8];
    const rent_info = accounts[9];

    if (!maker.isSigner()) return ProgramError.InvalidAcctData;

    const offer_seeds = &[_][]const u8{ "offer", &maker.id().bytes, std.mem.asBytes(&data.id) };

    const offer_pda = try PublicKey.findProgramAddress(offer_seeds, program_id.*);
    const offer_id = offer_pda.address;
    const offer_bump = offer_pda.bump_seed[0];

    const signer_seeds = [_][]const []const u8{
        &[_][]const u8{
            "offer",
            &maker.id().bytes,
            std.mem.asBytes(&data.id),
            &[_]u8{offer_bump},
        },
    };

    if (!PublicKey.equals(offer_info.id(), offer_id)) return ProgramError.InvalidAcctData;

    const expected_vault_pda = try sol_lib.associated_token_account.getAssociatedTokenAccountAddressAndBumpSeed(offer_info.id(), token_mint_a.id(), token_program.id());

    const expected_vault_pda_address = expected_vault_pda.address;

    if (!PublicKey.equals(expected_vault_pda_address, vault.id())) return ProgramError.InvalidAcctData;

    const offer = Offer{ .bump = offer_bump, .maker = maker.id(), .id = data.id, .token_mint_a = token_mint_a.id(), .token_mint_b = token_mint_b.id(), .token_b_wanted_amount = data.token_b_wanted_amount };

    const size = @sizeOf(Offer);
    const rent = try Rent.get();
    const lamports_required = rent.getMinimumBalance(size);

    sol_lib.system.createAccount(.{ .from = payer.info(), .to = offer_info.info(), .lamports = lamports_required, .space = size, .owner_id = program_id.*, .seeds = signer_seeds[0..] }) catch |e| return switch (e) {
        else => error.Unexpected,
    };

    sol_lib.associated_token_account.createAccount(.{
        .funder = payer.info(),
        .account = vault.info(),
        .owner = offer_info.info(),
        .mint = token_mint_a.info(),
        .system_program = system_program.info(),
        .token_program = token_program.info(),
        .rent = rent_info.info(),
    }) catch |e| return switch (e) {
        else => error.Unexpected,
    };

    sol_lib.token.transfer(.{
        .from = maker_token_account_a.info(),
        .to = vault.info(),
        .amount = data.token_a_offered_amount,
        .authority = .{ .single = maker.info() },
    }) catch |e| return switch (e) {
        else => error.Unexpected,
    };

    const valut_token_account = sol_lib.token.Account.decode(vault.data()) catch |e| return switch (e) {
        else => error.Unexpected,
    };

    const vault_token_account_amount = valut_token_account.amount;

    if (vault_token_account_amount != data.token_a_offered_amount) return ProgramError.InvalidAcctData;

    const bytes = std.mem.asBytes(&offer);
    @memcpy(offer_info.data()[0..bytes.len], bytes);
}

pub fn take_offer(program_id: *PublicKey, accounts: []Account) ProgramError!void {
    if (!(accounts.len == 10)) return ProgramError.InvalidAcctData;

    const offer_info = accounts[0];
    const token_mint_a = accounts[1];
    const token_mint_b = accounts[2];
    const maker_token_account_b = accounts[3];
    const taker_token_account_a = accounts[4];
    const taker_token_account_b = accounts[5];
    const vault = accounts[6];
    const maker = accounts[7];
    const taker = accounts[8];
    const payer = accounts[9];
    const token_program = accounts[10];
    const system_program = accounts[11];
    const rent_info = accounts[12];

    if (!taker.isSigner()) return ProgramError.InvalidAcctData;

    const offer_bytes = offer_info.data()[0..@sizeOf(Offer)];
    var offer: Offer = undefined;
    @memcpy(std.mem.asBytes(&offer), offer_bytes);

    if (!PublicKey.equals(offer.maker, maker.id())) return ProgramError.PublicKeyMismatch;
    if (!PublicKey.equals(offer.token_mint_a, token_mint_a.id())) return ProgramError.PublicKeyMismatch;
    if (!PublicKey.equals(offer.token_mint_b, token_mint_b.id())) return ProgramError.PublicKeyMismatch;

    const signer_seeds = [_][]const []const u8{
        &[_][]const u8{
            "offer",
            &maker.id().bytes,
            std.mem.asBytes(&offer.id),
            &[_]u8{offer.bump},
        },
    };

    const offer_key = PublicKey.createProgramAddress(
        &.{
            "offer",
            &maker.id().bytes,
            std.mem.asBytes(&offer.id),
            &[_]u8{offer.bump},
        },
        program_id.*,
    ) catch |e| return switch (e) {
        else => error.Unexpected,
    };

    if (!PublicKey.equals(offer_key, offer_info.id())) return ProgramError.PublicKeyMismatch;

    try assert_ata(maker_token_account_b.id(), maker.id(), token_mint_b.id());

    try assert_ata(taker_token_account_a.id(), taker.id(), token_mint_a.id());

    if (taker_token_account_a.lamports().* == 0) {
        // TODO; improve error handling
        sol_lib.associated_token_account.createAccount(.{
            .funder = payer.info(),
            .account = taker_token_account_a.info(),
            .owner = taker.info(),
            .mint = token_mint_a.info(),
            .system_program = system_program.info(),
            .token_program = token_program.info(),
            .rent = rent_info.info(),
        }) catch return error.Unexpected;
    }

    if (maker_token_account_b.lamports().* == 0) {
        sol_lib.associated_token_account.createAccount(.{
            .funder = payer.info(),
            .account = maker_token_account_b.info(),
            .owner = maker.info(),
            .mint = token_mint_b.info(),
            .system_program = system_program.info(),
            .token_program = token_program.info(),
            .rent = rent_info.info(),
        }) catch return error.Unexpected;
    }

    const vault_amount_a = (sol_lib.token.Account.decode(vault.data()) catch return error.Unexpected).amount;

    const taker_amount_a_before_transfer = (sol_lib.token.Account.decode(taker_token_account_a.data()) catch return error.Unexpected).amount;

    sol_lib.token.transfer(.{
        .from = taker_token_account_b.info(),
        .to = maker_token_account_b.info(),
        .amount = offer.token_b_wanted_amount,
        .authority = .{ .single = taker.info() },
    }) catch return error.Unexpected;

    sol_lib.token.transfer(.{ .from = vault.info(), .to = taker_token_account_a.info(), .amount = vault_amount_a, .authority = .{ .multiple = &[_]Account.Info{
        offer_info.info(),
        taker.info(),
    } }, .seeds = signer_seeds[0..] }) catch return error.Unexpected;

    const taker_amount_a = (sol_lib.token.Account.decode(taker_token_account_a.data()) catch return error.Unexpected).amount;

    const maker_amount_b = (sol_lib.token.Account.decode(maker_token_account_b.data()) catch return error.Unexpected).amount;

    if (taker_amount_a != taker_amount_a_before_transfer + vault_amount_a) return ProgramError.InvalidAcctData;

    if (maker_amount_b != taker_amount_a_before_transfer + offer.token_b_wanted_amount) return ProgramError.InvalidAcctData;

    sol_lib.token.closeAccount(.{ .account = vault.info(), .account_to_receive_remaining_tokens = taker.info(), .owner = offer_info.info(), .seeds = signer_seeds[0..] }) catch return error.Unexpected;

    const lamports = offer_info.lamports().*;
    offer_info.lamports().* -= lamports;
    payer.lamports().* += lamports;

    offer_info.realloc(0) catch return error.Unexpected;

    offer_info.assign(system_program.id());
}

pub fn assert_ata(ata: PublicKey, owner: PublicKey, mint: PublicKey) ProgramError!void {
    const expected_pda = try sol_lib.associated_token_account.getAssociatedTokenAccountAddressAndBumpSeed(owner, mint, sol_lib.token.id);
    const expected_pda_address = expected_pda.address;
    if (!PublicKey.equals(expected_pda_address, ata)) return ProgramError.InvalidAcctData;
}
