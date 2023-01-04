cmake_minimum_required(VERSION 3.23)

project(zertz
    VERSION 0.1.0
    DESCRIPTION "A zertz board game: The GIMP Project")

# Define C and C++ standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# For the purpose to use LSP for C++
set(CMAKE_EXPORT_COMPILE_COMMANDS Yes)

# Change binary location into the bin directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Include FetchContent
include(FetchContent)

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(COMPILER_FLAGS -O3 -Wall -Wextra -Wpedantic)
elseif()
    set(COMPILER_FLAGS -O0 -ggdb -Wall -Wextra -Wpedantic)
endif()

# Include Raylib library
set(RAYLIB_VERSION 4.2.0)
find_package(raylib ${RAYLIB_VERSION} QUIET) # Requires at least version 4.0
if (NOT raylib_FOUND)
    FetchContent_Declare(
        raylib
        GIT_REPOSITORY https://github.com/raysan5/raylib.git
        GIT_TAG        main
    )
    FetchContent_GetProperties(raylib)
    if (NOT raylib_POPULATED)
        set(FETCHCONTENT_QUIET NO)
        FetchContent_Populate(raylib)
        set(BUILD_EXAMPLES OFF CACHE BOOL "" FORCE) # don't build the supplied examples
        add_subdirectory(${raylib_SOURCE_DIR} ${raylib_BINARY_DIR})
    endif()
endif()

set(SRC_FILES src/main.cc src/Board.cc)

add_executable(zertz ${SRC_FILES})
target_link_libraries(zertz raylib)
target_compile_options(zertz PUBLIC ${COMPILER_FLAGS})

# Checks if OSX and links appropriate frameworks (only required on MacOS)
if (APPLE)
    target_link_libraries(${PROJECT_NAME} "-framework IOKit")
    target_link_libraries(${PROJECT_NAME} "-framework Cocoa")
    target_link_libraries(${PROJECT_NAME} "-framework OpenGL")
endif()

FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        main
)
FetchContent_MakeAvailable(googletest)

# Settings for testing zertz
set(TEST_FILES tests/zertz_test.cc tests/Test_UnionFind.cc)

enable_testing()
add_executable(zertz_test ${TEST_FILES})
target_link_libraries(zertz_test GTest::gtest_main)
target_include_directories(zertz_test PRIVATE ${CMAKE_SOURCE_DIR}/src)
target_compile_options(zertz_test PUBLIC ${COMPILER_FLAGS})

include(GoogleTest)
gtest_discover_tests(zertz_test)
