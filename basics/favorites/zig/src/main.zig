const sol = @import("solana_program_sdk");
const sol_lib = @import("solana_program_library");
const std = @import("std");
const PublicKey = sol.public_key.PublicKey;
const Account = sol.account.Account;
const Rent = sol.rent.Rent;
const Context = sol.context.Context;

export fn entrypoint(input: [*]u8) u64 {
    var context = Context.load(input) catch return 1;

    processInstruction(context.program_id, context.accounts[0..context.num_accounts], context.data) catch |err| return @intFromError(err);

    return 0;
}

pub const ProgramError = error{ InvalidIxData, InvalidAcctData, Unexpected };

pub const Favorites = struct { number: u64, color: [32]u8, hobbies: [4][32]u8 };

pub const InstructionType = enum(u8) { create, get };

pub fn processInstruction(program_id: *PublicKey, accounts: []Account, data: []const u8) ProgramError!void {
    const instruction_type: *const InstructionType = @ptrCast(data);

    switch (instruction_type.*) {
        InstructionType.create => {
            const create_data: *align(1) const Favorites = @ptrCast(data[1..]);

            try create_pda_ix(program_id, accounts, create_data.*);
        },
        InstructionType.get => {
            try get_pda_ix(program_id, accounts);
        },
    }
}

pub fn create_pda_ix(program_id: *PublicKey, accounts: []Account, data: Favorites) ProgramError!void {
    if (!(accounts.len == 3)) return ProgramError.InvalidAcctData;

    const user = accounts[0];
    const favtorites_account = accounts[1];
    const system_program = accounts[2];

    if (!user.isSigner()) return ProgramError.InvalidAcctData;
    if (favtorites_account.dataLen() != 0) return ProgramError.InvalidAcctData;
    if (!PublicKey.equals(system_program.id(), sol_lib.system.id)) return ProgramError.InvalidAcctData;

    const seeds = &[_][]const u8{ "favorites", &user.id().bytes };

    const pda_result = try PublicKey.findProgramAddress(seeds, program_id.*);
    const favorites_pda = pda_result.address;
    const favorites_bump = pda_result.bump_seed[0];

    const signer_seeds = [_][]const []const u8{
        &[_][]const u8{
            "favorites",
            &user.id().bytes,
            &[_]u8{favorites_bump},
        },
    };

    if (!PublicKey.equals(favorites_pda, favtorites_account.id())) return ProgramError.InvalidAcctData;

    if (favtorites_account.dataLen() == 0) {
        const space = @sizeOf(Favorites);
        const rent = try Rent.get();
        const lamports = rent.getMinimumBalance(space);

        sol_lib.system.createAccount(.{ .from = user.info(), .to = favtorites_account.info(), .lamports = lamports, .space = space, .owner_id = program_id.*, .seeds = signer_seeds[0..] }) catch |e| return switch (e) {
            error.InvalidIxData => error.InvalidIxData,
            error.InvalidAcctData => error.InvalidAcctData,
            else => error.Unexpected,
        };

        const bytes = std.mem.asBytes(&data);
        @memcpy(favtorites_account.data()[0..bytes.len], bytes);
    } else return ProgramError.InvalidAcctData;
}

pub fn get_pda_ix(program_id: *PublicKey, accounts: []Account) ProgramError!void {
    if (!(accounts.len == 2)) return ProgramError.InvalidAcctData;

    const user = accounts[0];
    const favtorites_account = accounts[1];

    if (!user.isSigner()) return ProgramError.InvalidAcctData;

    if (favtorites_account.dataLen() == 0) return ProgramError.InvalidAcctData;

    const seeds = &[_][]const u8{ "favorites", &user.id().bytes };
    const pda_result = try PublicKey.findProgramAddress(seeds, program_id.*);
    const favorites_pda = pda_result.address;

    if (!PublicKey.equals(favorites_pda, favtorites_account.id())) return ProgramError.InvalidAcctData;

    const favorites_bytes = favtorites_account.data()[0..@sizeOf(Favorites)];
    var favorites: Favorites = undefined;
    @memcpy(std.mem.asBytes(&favorites), favorites_bytes);

    const color_str = std.mem.sliceTo(favorites.color[0..], 0);
    var hobby_strs: [4][]const u8 = undefined;
    for (0..4) |i| {
        hobby_strs[i] = std.mem.sliceTo(favorites.hobbies[i][0..], 0);
    }

    sol.log.print("User {f}'s favorite number is {}, favorite color is: {s}, and their hobbies are {s}, {s}, {s}, {s}", .{ user.id(), favorites.number, color_str, hobby_strs[0], hobby_strs[1], hobby_strs[2], hobby_strs[3] });
}
