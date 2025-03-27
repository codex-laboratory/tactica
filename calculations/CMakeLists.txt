cmake_minimum_required(VERSION 3.15)
project(calculations)

set(CMAKE_CXX_STANDARD 17)

add_library(calc SHARED src/movements.cpp)

set_target_properties(calc PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/Release
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/Release
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/Release
)