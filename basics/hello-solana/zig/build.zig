const std = @import("std");
const solana = @import("solana_program_sdk");
const base58 = @import("base58");

pub fn build(b: *std.Build) !void {
    const target = b.resolveTargetQuery(solana.sbf_target);

    const optimize = .ReleaseFast;

    const program = b.addLibrary(.{ .name = "hello_world_program", .linkage = .dynamic, .root_module = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .optimize = optimize,
        .target = target,
    }) });

    _ = solana.buildProgram(b, program, target, optimize);
    b.installArtifact(program);

    const install_step = b.addInstallArtifact(program, .{ .dest_dir = .{ .override = .{ .custom = "../program-test/tests/fixtures" } } });
    b.getInstallStep().dependOn(&install_step.step);

    base58.generateProgramKeypair(b, program);
}
