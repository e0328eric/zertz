const std = @import("std");

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Board = @import("./Board.zig");
const Coordinate = @import("./Coordinate.zig");
const CoordinateIterator = @import("./Coordinate.zig").CoordinateIterator;
const Direction = @import("./Coordinate.zig").Direction;
const Marble = @import("./Board.zig").Marble;
const Ring = @import("./Board.zig").Ring;
const UnionFind = @import("./union_find.zig").UnionFind;

const Score = struct {
    white_count: usize,
    gray_count: usize,
    black_count: usize,

    fn default() @This() {
        return .{ .white_count = 0, .gray_count = 0, .black_count = 0 };
    }
};

const Player = enum(u1) {
    first = 0,
    second = 1,
};

const GameAction = enum(u1) {
    put_marble,
    catch_marble,
};

const GameError = Allocator.Error || error{
    InvalidRingToRemove,
    InvalidPuttingMarble,
};

// Game fields
allocator: Allocator,
board: Board,
components: UnionFind(Coordinate),
players: [2]Score,
current_player: Player,
action: GameAction,
// END fields

pub const Game = @This();
const Self = @This();

pub fn init(allocator: Allocator) !Self {
    var output: Self = undefined;
    var iter = CoordinateIterator.new(Coordinate.new(0, 0), Coordinate.new(6, 6), 6);
    var coord_list = try ArrayList(Coordinate).initCapacity(allocator, 49);
    defer coord_list.deinit();

    while (iter.next()) |coord| {
        try coord_list.append(coord);
    }

    output.allocator = allocator;

    output.board = try Board.init(allocator);
    errdefer output.board.deinit();

    output.components = try UnionFind(Coordinate).init(allocator, coord_list.items);
    errdefer output.components.deinit();

    output.players = [2]Score{ Score.default(), Score.default() };
    output.current_player = .first;
    output.action = .put_marble;

    output.calculateComponents();

    return output;
}

pub fn deinit(self: *Self) void {
    self.board.deinit();
    self.components.deinit();
}

inline fn validToRemoveRingHelper(
    self: *const Self,
    coord1: ?Coordinate,
    coord2: ?Coordinate,
) bool {
    const ring1 = self.board.get_option(coord1);
    const ring2 = self.board.get_option(coord2);

    return (ring1 == null or ring1.? == .empty) and
        (ring2 == null or ring2.? == .empty);
}

pub fn putMarble(
    self: *Self,
    put_coord: Coordinate,
    remove_coord: Coordinate,
    marble: Marble,
) GameError!void {
    if (self.board.get_raw(put_coord) == .vacant) {
        self.board.get_mut_raw(put_coord).* = Ring{ .occupied = marble };
    } else {
        return error.InvalidPuttingMarble;
    }

    try self.removeRing(remove_coord);
    try self.removeIsolatedIsland();
}

fn removeRing(self: *Self, coord: Coordinate) GameError!void {
    if (!self.validToRemoveRing(coord)) {
        return error.InvalidRingToRemove;
    }

    var ring = self.board.get_mut_raw(coord);
    if (ring.* == .vacant) {
        ring.* = .empty;
    } else {
        return error.InvalidRingToRemove;
    }

    self.calculateComponents();
}

fn validToRemoveRing(self: *const Self, coord: Coordinate) bool {
    const up_right = coord.adjacent(false, true, true, false);
    const up = coord.adjacent(false, false, true, false);
    const left = coord.adjacent(true, false, false, false);
    const left_down = coord.adjacent(true, false, false, true);
    const down = coord.adjacent(false, false, false, true);
    const right = coord.adjacent(false, true, false, false);

    return self.validToRemoveRingHelper(up_right, up) //
    or self.validToRemoveRingHelper(left, up) //
    or self.validToRemoveRingHelper(left, left_down) //
    or self.validToRemoveRingHelper(down, left_down) //
    or self.validToRemoveRingHelper(down, right) //
    or self.validToRemoveRingHelper(up_right, right);
}

// Use special coordinate to union empty rings
const main_empty_coord = Coordinate.new(6, 0);

fn calculateComponents(self: *Self) void {
    self.components.clear();

    var right_coord: Coordinate = undefined;
    var up_coord: Coordinate = undefined;
    var up_right_coord: Coordinate = undefined;

    var iter = CoordinateIterator.new(Coordinate.new(0, 0), Coordinate.new(6, 6), 6);

    while (iter.next()) |coord| {
        if (self.board.get_raw(coord) == .empty) {
            self.components.unionBoth(coord, main_empty_coord);
        }

        right_coord = coord.raw_adjacent(false, true, false, false);
        up_coord = coord.raw_adjacent(false, false, true, false);
        up_right_coord = coord.raw_adjacent(false, true, true, false);

        if (self.board.get(right_coord)) |ring| {
            if (ring == .empty) {
                self.components.unionBoth(right_coord, main_empty_coord);
            } else if (self.board.get_raw(coord) != .empty) {
                self.components.unionBoth(coord, right_coord);
            }
        }

        if (self.board.get(up_coord)) |ring| {
            if (ring == .empty) {
                self.components.unionBoth(up_coord, main_empty_coord);
            } else if (self.board.get_raw(coord) != .empty) {
                self.components.unionBoth(coord, up_coord);
            }
        }

        if (self.board.get(up_right_coord)) |ring| {
            if (ring == .empty) {
                self.components.unionBoth(up_right_coord, main_empty_coord);
            } else if (self.board.get_raw(coord) != .empty) {
                self.components.unionBoth(coord, up_right_coord);
            }
        }
    }
}

fn removeIsolatedIsland(self: *Self) !void {
    const main_components = try self.components.getComponentRepresentors();
    defer main_components.deinit();

    var component_members: ArrayList(Coordinate) = undefined;
    var iter: CoordinateIterator = undefined;

    components: for (main_components.items) |main_coord| {
        if (self.board.get_raw(main_coord) == .empty) {
            continue :components;
        }

        component_members = try ArrayList(Coordinate).initCapacity(self.allocator, 49);
        defer component_members.deinit();

        iter = CoordinateIterator.new(Coordinate.new(0, 0), Coordinate.new(6, 6), 6);
        while (iter.next()) |coord| {
            if (self.components.find(main_coord) != self.components.find(coord)) {
                continue;
            }

            if (self.board.get_raw(coord) == .vacant) {
                continue :components;
            }

            try component_members.append(coord);
        }

        for (component_members.items) |coord| {
            switch (self.board.get_raw(coord)) {
                .occupied => |marble| switch (marble) {
                    .white => self.players[@intCast(usize, @enumToInt(self.current_player))].white_count += 1,
                    .gray => self.players[@intCast(usize, @enumToInt(self.current_player))].gray_count += 1,
                    .black => self.players[@intCast(usize, @enumToInt(self.current_player))].black_count += 1,
                },
                else => unreachable,
            }

            self.board.get_mut_raw(coord).* = .empty;
            self.components.unionBoth(coord, main_empty_coord);
        }
    }
}
