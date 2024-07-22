cmake_minimum_required(VERSION 3.29)
project(daedalus)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Vulkan REQUIRED)

add_subdirectory(third_party/SDL3)
#add_subdirectory(src)

add_executable(daedalus src/main.cpp)

target_include_directories(daedalus PRIVATE Vulkan::Vulkan SDL3::SDL3)
target_link_libraries(daedalus PRIVATE Vulkan::Vulkan SDL3::SDL3)