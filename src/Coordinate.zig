// Coordinate fields
x: usize,
y: usize,
// END fields

pub const Coordinate = @This();
const Self = @This();

pub fn new(x: usize, y: usize) Self {
    return .{ .x = x, .y = y };
}

pub fn raw_adjacent(
    self: Self,
    comptime left: bool,
    comptime right: bool,
    comptime up: bool,
    comptime down: bool,
) Self {
    var output = self;

    if (left) {
        output.x -= 1;
    }
    if (right) {
        output.x += 1;
    }
    if (up) {
        output.y += 1;
    }
    if (down) {
        output.y -= 1;
    }

    return output;
}

pub fn adjacent(
    self: Self,
    comptime left: bool,
    comptime right: bool,
    comptime up: bool,
    comptime down: bool,
) ?Self {
    var output = self;

    if (left) {
        if (output.x == 0) {
            return null;
        }
        output.x -= 1;
    }
    if (right) {
        output.x += 1;
    }
    if (up) {
        output.y += 1;
    }
    if (down) {
        if (output.y == 0) {
            return null;
        }
        output.y -= 1;
    }

    return output;
}

pub const Compare = enum {
    eq,
    neq,
    less,
    less_eq,
    great,
    great_eq,
};

pub fn cmp(self: Self, comptime compare: Compare, rhs: Self) bool {
    switch (compare) {
        .eq => return self.x == rhs.x and self.y == rhs.y,
        .neq => return !self.cmp(.eq, rhs),
        .less => {
            if (self.y == rhs.y) {
                return self.x < rhs.x;
            } else {
                return self.y < rhs.y;
            }
        },
        .less_eq => {
            if (self.y == rhs.y) {
                return self.x <= rhs.x;
            } else {
                return self.y <= rhs.y;
            }
        },
        .great => return !self.cmp(.less_eq, rhs),
        .great_eq => return !self.cmp(.less, rhs),
    }
}

pub const CoordinateIterator = struct {
    current: Coordinate,
    end: Coordinate,
    row_limit: usize,

    pub fn new(start: Coordinate, end: Coordinate, row_limit: usize) @This() {
        return .{ .current = start, .end = end, .row_limit = row_limit };
    }

    pub fn next(self: *@This()) ?Coordinate {
        if (self.current.cmp(.great, self.end)) {
            return null;
        }

        const output = self.current;

        if (self.current.x >= self.row_limit) {
            self.current.x = 0;
            self.current.y += 1;
        } else {
            self.current.x += 1;
        }

        return output;
    }
};
