const std = @import("std");
const mem = std.mem;

pub const Direction = packed struct {
    left: bool,
    right: bool,
    up: bool,
    down: bool,

    pub const left = Direction.new(0b0001);
    pub const right = Direction.new(0b0010);
    pub const up = Direction.new(0b0100);
    pub const down = Direction.new(0b1000);
    pub const up_right = Direction.new(0b0110);
    pub const left_down = Direction.new(0b1001);

    fn new(bitfield: u4) @This() {
        return @bitCast(@This(), bitfield);
    }
};

pub fn GeneralCoordinate(comptime T: type) type {
    switch (@typeInfo(T)) {
        .Int => {},
        else => @compileError("GeneralCoordinate fields must be an integer type."),
    }

    return struct {
        x: T,
        y: T,

        const Self = @This();

        pub fn new(x: T, y: T) Self {
            return Self{ .x = x, .y = y };
        }

        pub fn rawAdjacent(
            self: Self,
            comptime direction: Direction,
        ) Self {
            var output = self;

            if (direction.left) {
                output.x -= 1;
            }

            if (direction.right) {
                output.x += 1;
            }

            if (direction.up) {
                output.y += 1;
            }

            if (direction.down) {
                output.y -= 1;
            }

            return output;
        }

        pub fn adjacent(
            self: Self,
            comptime direction: Direction,
        ) ?Self {
            var output = self;

            if (direction.left) if (output.x == 0) return null;
            if (direction.down) if (output.y == 0) return null;

            return output.rawAdjacent(direction);
        }

        pub fn intoUsize(self: Self) usize {
            return @intCast(usize, self.x) + 9 * @intCast(usize, self.y);
        }

        pub fn cmp(self: Self, comptime operator: []const u8, other: Self) bool {
            if (comptime mem.eql(u8, operator, "==")) {
                return self.x == other.x and self.y == other.y;
            }
            if (comptime mem.eql(u8, operator, "!=")) {
                return self.x != other.x or self.y != other.y;
            }
            if (comptime mem.eql(u8, operator, "<")) {
                return if (self.y == other.y) self.x < other.x else self.y < other.y;
            }
            if (comptime mem.eql(u8, operator, "<=")) {
                return if (self.y == other.y) self.x <= other.x else self.y <= other.y;
            }
            if (comptime mem.eql(u8, operator, ">")) {
                return if (self.y == other.y) self.x > other.x else self.y > other.y;
            }
            if (comptime mem.eql(u8, operator, ">=")) {
                return if (self.y == other.y) self.x >= other.x else self.y >= other.y;
            }

            @compileError("invalid operator `" ++ operator ++ "` was found");
        }
    };
}

pub fn GeneralCoordinateIter(comptime T: type) type {
    switch (@typeInfo(T)) {
        .Int => {},
        else => @compileError("GeneralCoordinateIter fields must be an integer type."),
    }

    return struct {
        current: GeneralCoordinate(T),
        end: GeneralCoordinate(T),
        row_limit: T,

        const Self = @This();
        const Item = GeneralCoordinate(T);

        pub fn new() Self {
            return Self{
                .current = Coordinate.new(0, 0),
                .end = Coordinate.new(8, 8),
                .row_limit = 8,
            };
        }

        pub fn isEnd(self: Self) bool {
            return self.current.cmp(">", self.end);
        }

        pub fn next(self: *Self) ?Item {
            if (self.isEnd()) return null;

            var output = self.current;

            if (self.current.x >= self.row_limit) {
                self.current.x = 0;
                self.current.y += 1;
            } else {
                self.current.x += 1;
            }

            return output;
        }
    };
}

pub const Coordinate = GeneralCoordinate(usize);
pub const CoordinateIter = GeneralCoordinateIter(usize);
