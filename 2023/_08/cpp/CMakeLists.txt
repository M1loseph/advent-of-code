cmake_minimum_required(VERSION 3.28)

project(Advent_of_code_2023_day_8)

set(CMAKE_CXX_STANDARD 23)

set(HEADERS error.hpp graph.hpp graph_loader.hpp)
set(SOURCES graph.cpp graph_loader.cpp main.cpp)

add_executable(main ${HEADERS} ${SOURCES})
