const std = @import("std");
const coordinate = @import("./coordinate.zig");
const meta = std.meta;

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Board = @import("./board.zig").Board;
const BoardKind = @import("./board.zig").BoardKind;
const Coordinate = coordinate.Coordinate;
const CoordinateIter = coordinate.CoordinateIter;
const Direction = coordinate.Direction;
const Marble = @import("./board.zig").Marble;
const Ring = @import("./board.zig").Ring;
const UnionFind = @import("./union_find.zig").UnionFind;

const main_empty_coord = Coordinate.new(8, 0);

const MarbleCount = struct {
    white_count: usize,
    gray_count: usize,
    black_count: usize,

    fn new(white: usize, gray: usize, black: usize) @This() {
        return @This(){ .white_count = white, .gray_count = gray, .black_count = black };
    }

    fn inc(self: *@This(), marble: Marble) void {
        switch (marble) {
            inline else => |marble_name| @field(self, @tagName(marble_name) ++ "_count") += 1,
        }
    }

    fn dec(self: *@This(), marble: Marble) bool {
        switch (marble) {
            inline else => |marble_name| {
                var marble_count = &@field(self, @tagName(marble_name) ++ "_count");
                if (marble_count.* == 0) return false;
                marble_count.* -= 1;
            },
        }

        return true;
    }

    fn isWin(self: @This()) bool {
        return self.white_count >= 4 //
        or self.gray_count >= 5 //
        or self.black_count >= 6 //
        or (self.white_count >= 3 and self.gray_count >= 3 and self.black_count >= 3);
    }
};

pub const Player = enum(usize) {
    alice,
    bob,
    tie,

    fn changePlayer(self: *@This()) void {
        switch (self.*) {
            .alice => self.* = .bob,
            .bob => self.* = .alice,
            .tie => {},
        }
    }
};

pub const CatchableMove = struct {
    start_coord: Coordinate,
    catched_coord: Coordinate,
    marble_land_coord: Coordinate,
};

pub const GameState = union(enum) {
    check_is_catchable,
    found_sequential_move,
    put_marble,
    catch_marble,
    game_end: Player,
};

pub const GameError = std.mem.Allocator.Error || error{
    FailedToCatchMarble,
    InvalidPuttingMarble,
    InvalidRingToRemove,
};

// Fields for Game
allocator: Allocator,
board: Board,
board_replace_history: ArrayList(Board),
components: UnionFind(Coordinate),
current_player: Player,
game_state: GameState,
players_score: [2]MarbleCount,
repeat_count: usize,
total_marble: MarbleCount,
sequential_move_list: ?ArrayList(CatchableMove),
// END Fields

pub const Game = @This();
const Self = @This();

// ╭──────────────────────────────────────────────────────────╮
// │                      Basic Game Api                      │
// ╰──────────────────────────────────────────────────────────╯

pub fn init(allocator: Allocator, kind: BoardKind) GameError!Self {
    var output = Self{
        .allocator = allocator,
        .board = Board.new(kind),
        .current_player = .alice,
        .game_state = .put_marble,
        .players_score = [_]MarbleCount{ MarbleCount.new(0, 0, 0), MarbleCount.new(0, 0, 0) },
        .repeat_count = 0,
        .total_marble = MarbleCount.new(6, 8, 10),
        .sequential_move_list = null,
    };

    output.board_replace_history = try ArrayList(Board).initCapacity(allocator, 20);
    errdefer output.board_replace_history.deinit();

    var elems: [81]Coordinate = undefined;

    var iter = CoordinateIter.new();
    var i: usize = 0;
    while (iter.next()) |coord| {
        elems[i] = coord;
        i += 1;
    }

    output.components = try UnionFind(Coordinate).init(allocator, &elems);
    errdefer output.components.deinit();

    output.calculateCompoments();

    return output;
}

pub fn deinit(self: *Self) void {
    self.board_replace_history.deinit();
    self.components.deinit();
    if (self.sequential_move_list) |inner| {
        inner.deinit();
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                   Catching Marble Api                    │
// ╰──────────────────────────────────────────────────────────╯

pub fn catchMarble(self: *Self, catch_data: CatchableMove) GameError!void {
    const start_coord = catch_data.start_coord;
    const catched_coord = catch_data.catched_coord;
    const marble_land_coord = catch_data.marble_land_coord;

    switch (self.board.getRaw(catched_coord)) {
        .occupied => |marble| switch (marble) {
            .white => self.players_score[@enumToInt(self.current_player)].white_count += 1,
            .gray => self.players_score[@enumToInt(self.current_player)].gray_count += 1,
            .black => self.players_score[@enumToInt(self.current_player)].black_count += 1,
        },
        else => return error.FailedToCatchMarble,
    }

    self.board.getRawMut(marble_land_coord).* = self.board.getRaw(start_coord);
    self.board.getRawMut(start_coord).* = .vacant;
    self.board.getRawMut(catched_coord).* = .vacant;

    try self.board_replace_history.append(self.board);

    const list_catchable = self.listCatchableOnce(marble_land_coord);
    errdefer list_catchable.deinit();

    if (list_catchable.items.len == 0) {
        self.current_player.changePlayer();
        self.game_state = if (self.whoIsWin()) |winner| .{ .game_end = winner } else blk: {
            break :blk .found_sequential_move;
        };
        self.sequential_move_list = if (self.sequential_move_list) |inner| blk: {
            inner.deinit();
            break :blk null;
        } else null;
        list_catchable.deinit();
    } else {
        self.game_state = if (self.whoIsWin()) |winner| .{ .game_end = winner } else blk: {
            break :blk .found_sequential_move;
        };
        self.sequential_move_list = if (self.sequential_move_list) |inner| blk: {
            inner.deinit();
            break :blk list_catchable;
        } else list_catchable;
    }
}

pub fn listAllCatchable(self: Self) GameError!ArrayList(CatchableMove) {
    var output = try ArrayList(CatchableMove).initCapacity(self.allocator, 81);
    errdefer output.deinit();

    var iter = CoordinateIter.new();
    while (iter.next()) |coord| {
        switch (self.board.getRaw(coord)) {
            .occupied => {
                const one_catchable = try self.listAllCatchableOnce(coord);
                defer one_catchable.deinit();
                try output.appendSlice(one_catchable.items);
            },
            else => {},
        }
    }

    return output;
}

fn listAllCatchableOnce(self: Self, coord: Coordinate) GameError!ArrayList(CatchableMove) {
    var output = try ArrayList(CatchableMove).initCapacity(self.allocator, 6);
    errdefer output.deinit();

    self.appendOccupiedCoordinate(&output, coord, Direction.up_right);
    self.appendOccupiedCoordinate(&output, coord, Direction.up);
    self.appendOccupiedCoordinate(&output, coord, Direction.left);
    self.appendOccupiedCoordinate(&output, coord, Direction.left_down);
    self.appendOccupiedCoordinate(&output, coord, Direction.down);
    self.appendOccupiedCoordinate(&output, coord, Direction.right);

    return output;
}

inline fn appendOccupiedCoordinate(
    self: Self,
    coord_list: *ArrayList(CatchableMove),
    coord: Coordinate,
    comptime direction: Direction,
) GameError!void {
    const catched_coord = coord.adjacent(direction) orelse return;
    const marble_land_coord = catched_coord.adjacent(direction) orelse return;

    if (self.board.get(catched_coord)) |catched_ring| {
        switch (catched_ring) {
            .occupied => {
                if (self.board.get(marble_land_coord)) |marble_land_ring| {
                    switch (marble_land_ring) {
                        .vacant => {
                            try coord_list.append(.{
                                .start_coord = coord,
                                .catched_coord = catched_coord,
                                .marble_land_coord = marble_land_coord,
                            });
                        },
                        else => {},
                    }
                }
            },
            else => {},
        }
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                    Putting Marble Api                    │
// ╰──────────────────────────────────────────────────────────╯

pub fn putMarble(
    self: *Self,
    put_coord: Coordinate,
    remove_coord: Coordinate,
    marble: Marble,
) GameError!void {
    if (self.board.getRaw(put_coord) == .vacant) {
        if (!self.total_marble.dec(marble) //
        and !self.players_score[@enumToInt(self.current_player)].dec(marble)) {
            return error.InvalidPuttingMarble;
        }

        self.board.getRawMut(put_coord).* = .{ .occupied = marble };
    } else {
        return error.InvalidPuttingMarble;
    }

    try self.removeRing(remove_coord);
    try self.removeIsolatedIsland();
    self.current_player.changePlayer();
    self.game_state = .check_is_catchable;
}

fn removeRing(self: *Self, coord: Coordinate) GameError!void {
    const list_removable = self.collectRemovableRings();
    defer list_removable.deinit();

    if (list_removable.items.len == 0) {
        if (!self.validToRemoveRing(coord)) return error.InvalidRingToRemove;

        self.board_replace_history.clearRetainingCapacity();
        self.repeat_count = 0;

        const ring = self.board.getRawMut(coord);
        if (ring.* != .vacant) return error.InvalidRingToRemove;
        ring.* = .empty;
        self.calculateCompoments();
    }
}

pub fn collectRemovableRings(self: Self) GameError!ArrayList(Coordinate) {
    var output = try ArrayList(Coordinate).initCapacity(self.allocator, 81);
    errdefer output.deinit();

    var iter = CoordinateIter.new();
    while (iter.next()) |coord| {
        if (self.validToRemoveRing(coord)) try output.append(coord);
    }

    return output;
}

inline fn validToRemoveRingHelper(
    self: Self,
    coord: Coordinate,
    comptime direction1: Direction,
    comptime direction2: Direction,
) bool {
    const coord1 = coord.adjacent(direction1);
    const coord2 = coord.adjacent(direction2);
    const ring1 = self.board.getOptional(coord1) orelse return false;
    const ring2 = self.board.getOptional(coord2) orelse return false;

    return ring1 == .empty and ring2 == .empty;
}

fn validToRemoveRing(self: Self, coord: Coordinate) bool {
    return self.validToRemoveRingHelper(coord, Direction.up_right, Direction.up) //
    or self.validToRemoveRingHelper(coord, Direction.left, Direction.up) //
    or self.validToRemoveRingHelper(coord, Direction.left, Direction.left_down) //
    or self.validToRemoveRingHelper(coord, Direction.down, Direction.left_down) //
    or self.validToRemoveRingHelper(coord, Direction.down, Direction.right) //
    or self.validToRemoveRingHelper(coord, Direction.up_right, Direction.right);
}

pub fn calculateCompoments(self: *Self) void {
    self.components.clear();

    var right_coord = undefined;
    var up_coord = undefined;
    var up_right_coord = undefined;
    var iter = CoordinateIter.new();

    while (iter.next()) |coord| {
        if (self.board.getRaw(coord).isEq(.empty)) {
            self.components.unionBoth(coord, main_empty_coord);
        }

        right_coord = coord.rawAdjacent(false, true, false, false);
        up_coord = coord.rawAdjacent(false, false, true, false);
        up_right_coord = coord.rawAdjacent(false, true, true, false);

        if (self.board.get(right_coord)) |ring| {
            switch (ring) {
                .empty => self.components.unionBoth(right_coord, main_empty_coord),
                else => if (ring.isEq(self.board.getRaw(coord))) blk: {
                    break :blk self.components.unionBoth(coord, right_coord);
                },
            }
        }

        if (self.board.get(up_coord)) |ring| {
            switch (ring) {
                .empty => self.components.unionBoth(up_coord, main_empty_coord),
                else => if (ring.isEq(self.board.getRaw(coord))) blk: {
                    break :blk self.components.unionBoth(coord, up_coord);
                },
            }
        }

        if (self.board.get(up_right_coord)) |ring| {
            switch (ring) {
                .empty => self.components.unionBoth(up_right_coord, main_empty_coord),
                else => if (ring.isEq(self.board.getRaw(coord))) blk: {
                    break :blk self.components.unionBoth(coord, up_right_coord);
                },
            }
        }
    }
}

fn removeIsolatedIsland(self: *Self) GameError!void {
    const main_components = try self.components.getComponentRepresentors();
    defer main_components.deinit();

    components: for (main_components.items) |main_coord| {
        if (self.board.getRaw(main_coord) == .empty) continue :components;

        var component_members = try ArrayList(Coordinate).initCapacity(self.allocator, 81);
        defer component_members.deinit();

        var iter = CoordinateIter.new();
        while (iter.next()) |coord| {
            if (self.components.find(main_coord) != self.components.find(coord)) continue;
            if (self.board.getRaw(coord) == .vacant) continue :components;

            try component_members.append(coord);
        }

        for (component_members.items) |coord| {
            switch (self.board.getRaw(coord)) {
                .occupied => |marble| self.players_score[@enumToInt(self.current_player)].inc(marble),
                else => unreachable,
            }

            self.board.getRawMut(coord).* = .empty;
            self.components.unionBoth(coord, main_empty_coord);
        }
    }
}

// ╭──────────────────────────────────────────────────────────╮
// │                     Check who is win                     │
// ╰──────────────────────────────────────────────────────────╯

fn whoIsWin(self: *Self) ?Player {
    if (self.players_score[@enumToInt(.alice)].isWin()) return .alice;
    if (self.players_score[@enumToInt(.bob)].isWin()) return .bob;

    for (self.board_replace_history.items) |prev_board| {
        if (meta.eql(self.board, prev_board)) {
            break;
        }
    } else {
        return null;
    }

    self.repeat_count += 1;
    if (self.repeat_count >= 3) return .tie;

    return null;
}
