const std = @import("std");
const sort = std.sort;

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

pub fn UnionFind(comptime T: type) type {
    return struct {
        allocator: Allocator,
        inner: AutoHashMap(T, usize),
        reverse: AutoHashMap(usize, T),
        parent_data: ArrayList(usize),
        rank_data: ArrayList(usize),

        const Self = @This();

        pub fn init(allocator: Allocator, elems: []const T) !Self {
            var output: Self = undefined;

            output.allocator = allocator;
            output.inner = AutoHashMap(T, usize).init(allocator);
            errdefer output.inner.deinit();

            output.reverse = AutoHashMap(usize, T).init(allocator);
            errdefer output.reverse.deinit();

            output.parent_data = try ArrayList(usize).initCapacity(allocator, elems.len);
            errdefer output.parent_data.deinit();

            output.rank_data = try ArrayList(usize).initCapacity(allocator, elems.len);
            errdefer output.rank_data.deinit();

            // Rreserve both HashMap capacity enough to use
            try output.inner.ensureTotalCapacity(@intCast(u32, elems.len));
            try output.reverse.ensureTotalCapacity(@intCast(u32, elems.len));

            for (elems) |elem, idx| {
                try output.inner.put(elem, idx);
                try output.reverse.put(idx, elem);
                try output.parent_data.append(idx);
                try output.rank_data.append(0);
            }

            return output;
        }

        pub fn deinit(self: *Self) void {
            self.inner.deinit();
            self.reverse.deinit();
            self.parent_data.deinit();
            self.rank_data.deinit();
        }

        pub fn unionBoth(self: *Self, x: T, y: T) void {
            const x_location = self.find(x);
            const y_location = self.find(y);

            if (x_location == y_location) {
                return;
            }

            if (self.rank_data.items[x_location] < self.rank_data.items[y_location]) {
                self.parent_data.items[x_location] = y_location;
            } else {
                self.parent_data.items[y_location] = x_location;

                if (self.rank_data.items[x_location] == self.rank_data.items[y_location]) {
                    self.rank_data.items[x_location] += 1;
                }
            }
        }

        pub fn find(self: *Self, x: T) usize {
            const x_location = self.inner.get(x).?;
            return self.findParent(x_location);
        }

        pub fn clear(self: *Self) void {
            var key_iter = self.inner.keyIterator();

            var idx: usize = 0;
            while (key_iter.next()) |elem| {
                self.inner.getPtr(elem.*).?.* = idx;
                self.reverse.getPtr(idx).?.* = elem.*;
                self.parent_data.items[idx] = idx;
                idx += 1;
            }

            for (self.rank_data.items) |*data| {
                data.* = 0;
            }
        }

        pub fn getComponentRepresentors(self: *Self) !ArrayList(T) {
            var parent_data = try self.parent_data.clone();
            defer parent_data.deinit();

            sort.sort(usize, parent_data.items, {}, comptime sort.asc(usize));

            var i: usize = 0;
            var j: usize = 0;
            while (j < parent_data.items.len) : (j += 1) {
                if (parent_data.items[i] == parent_data.items[j]) {
                    continue;
                }
                i += 1;
                parent_data.items[i] = parent_data.items[j];
            }
            parent_data.shrinkRetainingCapacity(i + 1);

            var output = try ArrayList(T).initCapacity(self.allocator, self.inner.count());
            errdefer output.deinit();

            for (parent_data.items) |pd| {
                try output.append(self.reverse.get(pd).?);
            }

            return output;
        }

        fn findParent(self: *Self, location: usize) usize {
            return if (self.parent_data.items[location] == location)
                location
            else blk: {
                const root_parent = self.findParent(self.parent_data.items[location]);
                self.parent_data.items[location] = root_parent;
                break :blk root_parent;
            };
        }
    };
}

// Testing UnionFind
const test_allocator = std.testing.allocator;
const expectEqual = std.testing.expectEqual;

test "test unionfind data structure" {
    const elems = [_]i32{ 1, 2, 3, 4, 5, 6, 7, 8 };
    var union_find = try UnionFind(i32).init(test_allocator, &elems);
    defer union_find.deinit();

    union_find.unionBoth(1, 2);
    union_find.unionBoth(4, 5);
    union_find.unionBoth(6, 1);
    union_find.unionBoth(3, 7);
    union_find.unionBoth(7, 8);
    union_find.unionBoth(2, 5);

    try expectEqual(union_find.find(1), union_find.find(6));
    try expectEqual(union_find.find(2), union_find.find(6));
    try expectEqual(union_find.find(3), union_find.find(3));
    try expectEqual(union_find.find(4), union_find.find(6));
    try expectEqual(union_find.find(5), union_find.find(6));
    try expectEqual(union_find.find(6), union_find.find(6));
    try expectEqual(union_find.find(7), union_find.find(3));
    try expectEqual(union_find.find(8), union_find.find(3));

    union_find.clear();

    union_find.unionBoth(2, 3);
    union_find.unionBoth(1, 5);
    union_find.unionBoth(6, 7);
    union_find.unionBoth(7, 5);

    try expectEqual(union_find.find(1), union_find.find(1));
    try expectEqual(union_find.find(2), union_find.find(2));
    try expectEqual(union_find.find(3), union_find.find(2));
    try expectEqual(union_find.find(4), union_find.find(4));
    try expectEqual(union_find.find(5), union_find.find(1));
    try expectEqual(union_find.find(6), union_find.find(1));
    try expectEqual(union_find.find(7), union_find.find(1));
    try expectEqual(union_find.find(8), union_find.find(8));
}
