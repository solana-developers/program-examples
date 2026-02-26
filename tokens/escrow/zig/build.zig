const std = @import("std");
const solana = @import("solana_program_sdk");
const base58 = @import("base58");

pub fn build(b: *std.Build) !void {
    const target = b.resolveTargetQuery(solana.sbf_target);
    const optimize = .ReleaseFast;

    const dep_opts = .{ .target = target, .optimize = optimize };

    const solana_lib_dep = b.dependency("solana_program_library", dep_opts);
    const solana_lib_mod = solana_lib_dep.module("solana_program_library");

    const program = b.addLibrary(.{ .name = "escrow_program", .linkage = .dynamic, .root_module = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .optimize = optimize,
        .target = target,
    }) });

    program.root_module.addImport("solana_program_library", solana_lib_mod);

    _ = solana.buildProgram(b, program, target, optimize);
    b.installArtifact(program);

    const install_step = b.addInstallArtifact(program, .{ .dest_dir = .{ .override = .{ .custom = "../program-test/tests/fixtures" } } });
    b.getInstallStep().dependOn(&install_step.step);

    base58.generateProgramKeypair(b, program);
}
