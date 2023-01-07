const std = @import("std");

const Coordinate = @import("./Coordinate.zig");
const Game = @import("./Game.zig");
const Marble = @import("./Board.zig").Marble;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const leaked = gpa.deinit();
        if (leaked) @panic("some memories are leaking!!!");
    }

    var zertz = try Game.init(allocator);
    defer zertz.deinit();

    std.debug.print("{}\n", .{zertz.board});

    try testGame(&zertz, Coordinate.new(0, 0), Coordinate.new(6, 6), .white);
    try testGame(&zertz, Coordinate.new(3, 0), Coordinate.new(6, 5), .black);
    try testGame(&zertz, Coordinate.new(2, 2), Coordinate.new(2, 0), .gray);
    try testGame(&zertz, Coordinate.new(1, 0), Coordinate.new(4, 1), .gray);
    try testGame(&zertz, Coordinate.new(5, 6), Coordinate.new(5, 2), .gray);
    try testGame(&zertz, Coordinate.new(5, 5), Coordinate.new(4, 2), .white);
    try testGame(&zertz, Coordinate.new(3, 6), Coordinate.new(3, 1), .white);
    try testGame(&zertz, Coordinate.new(6, 4), Coordinate.new(5, 3), .white);
    try testGame(&zertz, Coordinate.new(0, 1), Coordinate.new(4, 3), .white);
    try testGame(&zertz, Coordinate.new(1, 1), Coordinate.new(5, 4), .gray);
    try testGame(&zertz, Coordinate.new(6, 3), Coordinate.new(0, 3), .black);
}

fn testGame(
    zertz: *Game,
    put_coord: Coordinate,
    remove_coord: Coordinate,
    marble: Marble,
) !void {
    try zertz.putMarble(put_coord, remove_coord, marble);
    std.debug.print("{}\n", .{zertz.board});
    std.debug.print("{any}\n", .{zertz.players});
}

test "zertz tests" {
    _ = @import("./union_find.zig");
}
