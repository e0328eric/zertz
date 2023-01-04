#ifndef ZERTS_UNION_FIND_HH_
#define ZERTS_UNION_FIND_HH_

#include <initializer_list>
#include <optional>
#include <unordered_map>
#include <vector>

namespace zertz
{
template <typename T>
class UnionFind
{
  public:
    UnionFind(std::initializer_list<T> elems);
    ~UnionFind() noexcept = default;

    // UnionFind is not copiable.
    UnionFind(const UnionFind&) = delete;
    UnionFind(UnionFind&&) = default;
    const UnionFind& operator=(const UnionFind&) = delete;
    UnionFind& operator=(UnionFind&&) = default;

    bool UnionBoth(const T& x, const T& y) noexcept;
    std::optional<size_t> Find(const T& x) noexcept;

  private:
    size_t findParentRecursive(size_t location) noexcept;

  private:
    std::unordered_map<T, size_t> mInner;
    std::vector<size_t> mParentData;
    std::vector<size_t> mRankData;
};
} // namespace zertz

//
// Implementation
//

template <typename T>
zertz::UnionFind<T>::UnionFind(std::initializer_list<T> elems)
    : mInner(elems.size()), mParentData(), mRankData()
{
    mParentData.reserve(elems.size());
    mRankData.reserve(elems.size());

    size_t idx = 0;
    for (const auto& elem : elems)
    {
        mInner.insert({elem, idx});
        mParentData.push_back(idx);
        idx += 1;
    }
}

template <typename T>
bool zertz::UnionFind<T>::UnionBoth(const T& x, const T& y) noexcept
{
    auto maybeXLocation = Find(x);
    auto maybeYLocation = Find(y);

    if (!maybeXLocation.has_value() || !maybeYLocation.has_value())
    {
        return false;
    }

    size_t xLocation = maybeXLocation.value();
    size_t yLocation = maybeYLocation.value();

    if (xLocation != yLocation)
    {
        if (mRankData[xLocation] < mRankData[yLocation])
        {
            mParentData[xLocation] = yLocation;
        }
        else
        {
            mParentData[yLocation] = xLocation;

            if (mRankData[xLocation] == mRankData[yLocation])
            {
                mRankData[xLocation] += 1;
            }
        }
    }

    return true;
}

template <typename T>
std::optional<size_t> zertz::UnionFind<T>::Find(const T& x) noexcept
{
    if (auto xLocation = mInner.find(x); xLocation != mInner.end())
    {
        return {findParentRecursive(xLocation->first)};
    }
    else
    {
        return {};
    }
}

template <typename T>
size_t zertz::UnionFind<T>::findParentRecursive(size_t location) noexcept
{
    if (mParentData[location] == location)
    {
        return location;
    }
    else
    {
        auto rootParent = findParentRecursive(mParentData[location]);
        mParentData[location] = rootParent;
        return rootParent;
    }
}

#endif // ZERTS_UNION_FIND_HH_
