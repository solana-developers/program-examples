const sol = @import("solana_program_sdk");

export fn entrypoint(input: [*]u8) u64 {
    const context = sol.context.Context.load(input) catch return 1;
    processInstruction(context.program_id) catch return 1;
    return 0;
}

fn processInstruction(program_id: *sol.public_key.PublicKey) !void {
    sol.log.log("Hello, Solana!");
    sol.log.print("Our program's Program ID: {f}", .{program_id});
}
