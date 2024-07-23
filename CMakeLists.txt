cmake_minimum_required(VERSION 3.29)
project(daedalus)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(third_party/SDL3)
#add_subdirectory(third_party/metal-cpp)
#add_subdirectory(src)

add_executable(daedalus src/main.cpp)

target_include_directories(daedalus PRIVATE
        SDL3::SDL3
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp-extensions
)
target_link_libraries(daedalus PRIVATE
        SDL3::SDL3
        "-framework Metal"
        "-framework Foundation"
        "-framework QuartzCore"
        "-framework MetalKit"
        "-framework AppKit"
)