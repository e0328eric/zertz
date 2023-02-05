const std = @import("std");
const zertz_core = @import("zertz-core");

const board_lib = zertz_core.board;

pub fn main() void {
    const board = board_lib.Board.new(board_lib.BoardKind.default());
    std.debug.print("{any}\n", .{board});
}
