cmake_minimum_required(VERSION 3.0)
project(buoyfinder_cpp)

# Compiler flags
if (MSVC)
    # Add /SUBSYSTEM:WINDOWS for GUI
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} /ENTRY:mainCRTStartup")
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++1y")
endif ()

# Files
set(HEADER_DIR ${CMAKE_SOURCE_DIR}/include)
set(SOURCE_DIR ${CMAKE_SOURCE_DIR}/src)

set(HEADERS ${HEADER_DIR}/buoyfinder/buoy_station.hpp)
set(SOURCES ${SOURCE_DIR}/buoy_station.cpp)

# Find Buoyfinder Rust library
include_directories(${HEADER_DIR} ${CMAKE_SOURCE_DIR}/../../include)
find_library(BUOYFINDER_DEBUG_LIBRARY NAMES buoyfinder.lib HINTS ${CMAKE_SOURCE_DIR}/../../target/debug)
find_library(BUOYFINDER_RELEASE_LIBRARY NAMES buoyfinder.lib HINTS ${CMAKE_SOURCE_DIR}/../../target/release)

add_library(buoyfinder SHARED ${HEADERS} ${SOURCES})

target_link_libraries(buoyfinder debug ${BUOYFINDER_DEBUG_LIBRARY} optimized ${BUOYFINDER_RELEASE_LIBRARY})

# Add the examples
#add_subdirectory(example)