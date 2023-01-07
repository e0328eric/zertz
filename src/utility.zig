const std = @import("std");
const os = std.os;

pub fn message(comptime msg: []const u8) noreturn {
    std.debug.print(msg ++ "\n", .{});
    os.exit(1);
}
