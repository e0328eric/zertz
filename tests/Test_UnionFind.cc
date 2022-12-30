#include <gtest/gtest.h>

#include "UnionFind.hh"

TEST(TestUnionFind, basic_action)
{
    zertz::UnionFind<int> union_find{1, 2, 3, 4, 5, 6, 7, 8};

    union_find.UnionBoth(1, 2);
    union_find.UnionBoth(4, 5);
    union_find.UnionBoth(6, 1);
    union_find.UnionBoth(3, 7);
    union_find.UnionBoth(7, 8);
    union_find.UnionBoth(2, 5);

    EXPECT_EQ(union_find.Find(1), union_find.Find(6));
    EXPECT_EQ(union_find.Find(2), union_find.Find(6));
    EXPECT_EQ(union_find.Find(3), union_find.Find(3));
    EXPECT_EQ(union_find.Find(4), union_find.Find(6));
    EXPECT_EQ(union_find.Find(5), union_find.Find(6));
    EXPECT_EQ(union_find.Find(6), union_find.Find(6));
    EXPECT_EQ(union_find.Find(7), union_find.Find(3));
    EXPECT_EQ(union_find.Find(8), union_find.Find(3));
}
