const sol = @import("solana_program_sdk");
const sol_lib = @import("solana_program_library");
const std = @import("std");
const Rent = sol.rent.Rent;
const PublicKey = sol.public_key.PublicKey;
const Account = sol.account.Account;

pub const ProgramError = error{ InvalidInstructionData, InvalidAccountData, Unexpected };

pub const AddressInfo = struct {
    name: [32]u8,
    house_number: u8,
    street: [64]u8,
    city: [32]u8,

    pub const SIZE = @sizeOf(AddressInfo);

    pub fn new(name: [32]u8, house_number: u8, street: [64]u8, city: [32]u8) AddressInfo {
        return AddressInfo{ .name = name, .house_number = house_number, .street = street, .city = city };
    }
};

export fn entrypoint(input: [*]u8) u64 {
    var context = sol.context.Context.load(input) catch return 1;
    processInstruction(context.program_id, context.accounts[0..context.num_accounts], context.data) catch |err| return @intFromError(err);
    return 0;
}

fn processInstruction(program_id: *PublicKey, accounts: []Account, data: []const u8) ProgramError!void {
    if (data.len < AddressInfo.SIZE) return ProgramError.InvalidInstructionData;
    if (accounts.len < 3) return ProgramError.InvalidAccountData;

    const address_info: AddressInfo = std.mem.bytesToValue(AddressInfo, data[0..AddressInfo.SIZE]);

    const address_info_account = accounts[0];
    const payer = accounts[1];
    const system_program = accounts[2];

    if (address_info_account.dataLen() != 0) return ProgramError.InvalidAccountData;
    if (!payer.isSigner()) return ProgramError.InvalidAccountData;
    if (!PublicKey.equals(system_program.id(), sol_lib.system.id)) return ProgramError.InvalidAccountData;

    const space = AddressInfo.SIZE;
    const rent = try Rent.get();
    const lamports = rent.getMinimumBalance(space);

    sol_lib.system.createAccount(.{
        .from = payer.info(),
        .to = address_info_account.info(),
        .lamports = lamports,
        .space = space,
        .owner_id = program_id.*,
    }) catch |e| return switch (e) {
        error.InvalidInstructionData => error.InvalidInstructionData,
        error.InvalidAccountData => error.InvalidAccountData,
        else => error.Unexpected,
    };

    const bytes = std.mem.asBytes(&address_info);

    if (address_info_account.dataLen() < bytes.len)
        return error.InvalidAccountData;

    @memcpy(address_info_account.data()[0..bytes.len], bytes);
}
