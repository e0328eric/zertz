const std = @import("std");

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Coordinate = @import("./Coordinate.zig");
const CoordinateIterator = @import("./Coordinate.zig").CoordinateIterator;

const assert = std.debug.assert;
const print = std.debug.print;

pub const Ring = union(enum) {
    empty,
    vacant,
    occupied: Marble,

    pub fn format(
        self: @This(),
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        try switch (self) {
            .empty => writer.print(".", .{}),
            .vacant => writer.print("O", .{}),
            .occupied => |marble| switch (marble) {
                .white => writer.print("\x1b[107;30m@\x1b[0m", .{}),
                .gray => writer.print("\x1b[100;30m@\x1b[0m", .{}),
                .black => writer.print("\x1b[97;40m@\x1b[0m", .{}),
            },
        };
    }
};

pub const Marble = enum(u2) {
    white,
    gray,
    black,
};

// Board fields
data: ArrayList(Ring),
// END fields

pub const Board = @This();
const Self = @This();

pub fn init(allocator: Allocator) !Self {
    var data = try ArrayList(Ring).initCapacity(allocator, 49);
    errdefer data.deinit();

    try data.appendNTimes(Ring.vacant, 49);

    var output = Self{ .data = data };
    var iter = CoordinateIterator.new(Coordinate.new(0, 0), Coordinate.new(6, 6), 6);

    while (iter.next()) |coord| {
        if (coord.x > coord.y + 3 or coord.y > coord.x + 3) {
            output.get_mut_raw(coord).* = Ring.empty;
        }
    }

    return output;
}

pub fn deinit(self: Self) void {
    self.data.deinit();
}

pub fn get_raw(self: *const Self, coord: Coordinate) Ring {
    assert(coord.x < 7 and coord.y < 7);
    return self.data.items[coord.x + 7 * coord.y];
}

pub fn get_mut_raw(self: *Self, coord: Coordinate) *Ring {
    assert(coord.x < 7 and coord.y < 7);
    return &self.data.items[coord.x + 7 * coord.y];
}

pub fn get(self: *const Self, coord: Coordinate) ?Ring {
    if (coord.x >= 7 or coord.y >= 7) {
        return null;
    }
    return self.data.items[coord.x + 7 * coord.y];
}

pub fn get_mut(self: *Self, coord: Coordinate) ?*Ring {
    if (coord.x >= 7 or coord.y >= 7) {
        return null;
    }
    return &self.data.items[coord.x + 7 * coord.y];
}

pub fn get_option(self: *const Self, coord: ?Coordinate) ?Ring {
    if (coord == null) {
        return null;
    }
    const coord_raw = coord.?;

    if (coord_raw.x >= 7 or coord_raw.y >= 7) {
        return null;
    }
    return self.data.items[coord_raw.x + 7 * coord_raw.y];
}

pub fn get_mut_option(self: *Self, coord: ?Coordinate) ?*Ring {
    if (coord == null) {
        return null;
    }
    const coord_raw = coord.?;

    if (coord_raw.x >= 7 or coord_raw.y >= 7) {
        return null;
    }
    return &self.data.items[coord_raw.x + 7 * coord_raw.y];
}

pub fn format(
    self: Self,
    comptime fmt: []const u8,
    options: std.fmt.FormatOptions,
    writer: anytype,
) !void {
    _ = fmt;
    _ = options;

    const loop_idx = [_]comptime_int{ 6, 5, 4, 3, 2, 1, 0 };
    comptime var i: comptime_int = undefined;
    inline for (loop_idx) |idx| {
        try writer.print("\n {}  " ++ " " ** (6 - idx), .{idx});
        i = 7 * idx;
        inline while (i < 7 * idx + 7) : (i += 1) {
            try writer.print("{} ", .{self.data.items[i]});
        }
    }
    try writer.print("\n\n            ", .{});

    i = 0;
    inline while (i < 7) : (i += 1) {
        try writer.print("{} ", .{i});
    }

    try writer.print("\n", .{});
}
