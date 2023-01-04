#include <cmath>
#include <raylib.h>

#include "Board.hh"

inline constexpr int screenWidth = 800;
inline constexpr int screenHeight = 450;

Vector2 coordToVec2(zertz::Coordinate coord)
{
    return {(float)coord.x * 60.0f - (float)coord.y * 30.0f + screenWidth / 2.7f,
            -std::sqrt(3.0f) * 30.0f * coord.y + screenHeight * 0.85f};
}

int main()
{
    InitWindow(screenWidth, screenHeight, "zertz_test");

    zertz::Board zertzBoard;

    SetTargetFPS(60);

    while (!WindowShouldClose())
    {
        BeginDrawing();

        ClearBackground(BLACK);

        for (size_t x = 0; x < 7; ++x)
        {
            for (size_t y = 0; y < 7; ++y)
            {
                const auto& ring = zertzBoard[zertz::Coordinate(x, y)];
                switch (ring.getType())
                {
                case zertz::Ring::RingType::None:
                    break;

                case zertz::Ring::RingType::Vacant:
                    DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 30, WHITE);
                    DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 20, BLACK);
                    break;

                case zertz::Ring::RingType::Occupied:
                    DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 30, WHITE);
                    DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 20, BLACK);
                    switch (ring.getMarble())
                    {
                    case zertz::Marble::White:
                        DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 10, WHITE);
                        break;

                    case zertz::Marble::Gray:
                        DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 10, GRAY);
                        break;

                    case zertz::Marble::Black:
                        DrawCircleV(coordToVec2(zertz::Coordinate(x, y)), 10, DARKGRAY);
                        break;
                    }
                    break;
                }
            }
        }

        EndDrawing();
    }

    CloseWindow();

    return 0;
}
