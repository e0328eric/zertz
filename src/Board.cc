#include "Board.hh"

using namespace zertz;

// Implementations for Coordinate
Coordinate::Coordinate(size_t x, size_t y) : x(x), y(y)
{
}

// Implementations for Ring
Ring::Ring(RingType type) : mType(type), mMarble()
{
}

Ring::Ring(Marble marble) : mType(Ring::RingType::Occupied), mMarble(marble)
{
}

bool Ring::hasMarble() const
{
    return mType == Ring::RingType::Occupied;
}

Ring::RingType Ring::getType() const
{
    return mType;
}

Marble Ring::getMarble() const
{
    return mMarble;
}

void Ring::setType(RingType type)
{
    mType = type;
}

void Ring::setMarble(Marble marble)
{
    mType = Ring::RingType::Occupied;
    mMarble = marble;
}

// Implementations for Board
Board::Board() : mData()
{
    mData.reserve(64);

    for (size_t x = 0; x < 7; ++x)
    {
        for (size_t y = 0; y < 7; ++y)
        {
            if (y <= x + 3 && y + 3 >= x)
            {
                (*this)[Coordinate(x, y)] = Ring(Ring::RingType::Vacant);
            }
            else
            {
                (*this)[Coordinate(x, y)] = Ring(Ring::RingType::None);
            }
        }
    }
}

Ring& Board::operator[](Coordinate coord)
{
    return mData[coord.x + 9 * coord.y];
}

const Ring& Board::operator[](Coordinate coord) const
{
    return mData[coord.x + 9 * coord.y];
}
