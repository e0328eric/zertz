const std = @import("std");
const Build = std.Build;

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{
        .preferred_optimize_mode = .ReleaseSafe,
    });

    const zertz_core_name = "zertz-core";
    const zertz_core_main = zertz_core_name ++ "/main.zig";
    const zertz_terminal_name = "zertz-terminal";
    const zertz_terminal_main = zertz_terminal_name ++ "/main.zig";

    const zertz_core_lib = b.addStaticLibrary(.{
        .name = zertz_core_name,
        .root_source_file = .{ .path = zertz_core_main },
        .target = target,
        .optimize = optimize,
        .version = .{ .major = 0, .minor = 1, .patch = 0 },
    });
    zertz_core_lib.install();

    const core_main_tests = b.addTest(.{
        .root_source_file = .{ .path = zertz_core_main },
        .target = target,
        .optimize = optimize,
    });

    const core_test_step = b.step("test-core", "Run library tests for zertz core");
    core_test_step.dependOn(&core_main_tests.step);

    const zertz_core_mod = b.createModule(.{
        .source_file = .{ .path = zertz_core_main },
    });

    const zertz_terminal_exe = b.addExecutable(.{
        .name = zertz_terminal_name,
        .root_source_file = .{ .path = zertz_terminal_main },
        .target = target,
        .optimize = optimize,
        .version = .{ .major = 0, .minor = 1, .patch = 0 },
    });
    zertz_terminal_exe.addModule(zertz_core_name, zertz_core_mod);
    zertz_terminal_exe.install();

    const zertz_terminal_run_cmd = zertz_terminal_exe.run();
    zertz_terminal_run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        zertz_terminal_run_cmd.addArgs(args);
    }

    const zertz_terminal_run_step = b.step("run-term", "Run the zertz terminal app");
    zertz_terminal_run_step.dependOn(&zertz_terminal_run_cmd.step);

    const zertz_terminal_tests = b.addTest(.{
        .root_source_file = .{ .path = zertz_terminal_main },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test-term", "Run unit tests for zertz terminal");
    test_step.dependOn(&zertz_terminal_tests.step);
}
