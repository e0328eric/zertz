#ifndef ZERTZ_BOARD_HH
#define ZERTZ_BOARD_HH

#include <cstdint>
#include <vector>

namespace zertz
{
class Coordinate
{
  public:
    Coordinate(size_t x, size_t y);

  public:
    size_t x;
    size_t y;
};

enum class Marble : uint8_t
{
    White,
    Gray,
    Black,
};

class Ring
{
  public:
    enum class RingType : int8_t;

    explicit Ring(RingType type);
    explicit Ring(Marble marble);

    ~Ring() noexcept = default;

    Ring(const Ring&) = default;
    Ring(Ring&&) noexcept = default;
    Ring& operator=(const Ring&) = default;
    Ring& operator=(Ring&&) = default;

    bool hasMarble() const;

    RingType getType() const;
    Marble getMarble() const;

    void setMarble(Marble marble);
    void setType(RingType type);

  public:
    enum class RingType : int8_t
    {
        None = -1,
        Vacant = 0,
        Occupied,
    };

  private:
    RingType mType;
    Marble mMarble;
};

class Board
{
  public:
    Board();
    ~Board() = default;

    Board(const Board&) = delete;
    Board(Board&&) noexcept = default;
    Board& operator=(const Board&) = delete;
    Board& operator=(Board&&) = default;

    Ring& operator[](Coordinate coord);
    const Ring& operator[](Coordinate coord) const;

  private:
    std::vector<Ring> mData;
};
} // namespace zertz

#endif // ZERTZ_BOARD_HH
