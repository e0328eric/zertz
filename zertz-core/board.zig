const std = @import("std");
const blueprint = @import("./board_blueprint.zig");

const Coordinate = @import("./coordinate.zig").Coordinate;

pub const Ring = union(enum) {
    vacant,
    empty,
    occupied: Marble,

    pub fn isEq(self: @This(), other: @This()) bool {
        return switch (self) {
            .empty => switch (other) {
                .empty => true,
                else => false,
            },
            else => switch (other) {
                .empty => false,
                else => true,
            },
        };
    }
};

pub const Marble = enum(u2) {
    white,
    gray,
    black,
};

pub const BoardKind = enum(u8) {
    rings37 = 37,
    rings40 = 40,
    rings43 = 43,
    rings44 = 44,
    rings48 = 48,
    rings61 = 61,

    pub fn default() @This() {
        return .rings37;
    }
};

pub const Board = struct {
    kind: BoardKind,
    data: [81]Ring,

    const Self = @This();

    pub fn new(kind: BoardKind) Self {
        var output = Self{ .kind = kind, .data = undefined };

        const board_blueprint = switch (kind) {
            .rings37 => blueprint.rings37_board,
            .rings40 => blueprint.rings40_board,
            .rings43 => blueprint.rings43_board,
            .rings44 => blueprint.rings44_board,
            .rings48 => blueprint.rings48_board,
            .rings61 => blueprint.rings61_board,
        };

        var row: usize = 0;
        var col: usize = 0;

        while (row < 9) : ({
            row += 1;
            col = 0;
        }) {
            while (col < 9) : (col += 1) {
                output.data[row * 9 + col] = if (board_blueprint[row][col] == 0) .empty else .vacant;
            }
        }

        return output;
    }

    pub fn getRaw(self: Self, coord: Coordinate) Ring {
        std.debug.assert(coord.x < 9 and coord.y < 9);
        return self.data[coord.intoUsize()];
    }

    pub fn getRawMut(self: Self, coord: Coordinate) *Ring {
        std.debug.assert(coord.x < 9 and coord.y < 9);
        return &self.data[coord.intoUsize()];
    }

    pub fn get(self: Self, coord: Coordinate) ?Ring {
        if (coord.x >= 9 or coord.y >= 9) return null;
        return self.data[coord.intoUsize()];
    }

    pub fn getMut(self: Self, coord: Coordinate) ?*Ring {
        if (coord.x >= 9 or coord.y >= 9) return null;
        return &self.data[coord.intoUsize()];
    }

    pub fn getOptional(self: Self, coord: ?Coordinate) ?Ring {
        if (coord == null or coord.?.x >= 9 or coord.?.y >= 9) return null;
        return self.data[coord.?.intoUsize()];
    }

    pub fn getOptionMut(self: Self, coord: ?Coordinate) ?*Ring {
        if (coord == null or coord.?.x >= 9 or coord.?.y >= 9) return null;
        return &self.data[coord.?.intoUsize()];
    }
};
