cmake_minimum_required(VERSION 3.29)
project(daedalus)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
#set(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR}/bin)
#set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
#set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

#add_definitions(-DAD_DEFINE_RES_ROOT_DIR-\"${CMAKE_SOURCE_DIR}/Resource\")

add_subdirectory(third_party/SDL3)
add_subdirectory(third_party/spdlog)
#add_subdirectory(third_party/metal-cpp)
#add_subdirectory(src)

add_executable(daedalus
        src/main.cpp
        src/game.cpp
        src/game.h
        src/renderer.cpp
        src/renderer.h
        src/camera.cpp
        src/camera.h
        src/path_tracer.cpp
        src/path_tracer.h
)

target_include_directories(daedalus PRIVATE
        SDL3::SDL3
        spdlog::spdlog
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp-extensions
)
if (IOS)
 target_link_libraries(daedalus PRIVATE
         SDL3::SDL3
         spdlog::spdlog
         "-framework Metal"
         "-framework Foundation"
         "-framework QuartzCore"
         "-framework MetalKit"
 )
else ()
 target_link_libraries(daedalus PRIVATE
         SDL3::SDL3
         spdlog::spdlog
         "-framework Metal"
         "-framework Foundation"
         "-framework QuartzCore"
         "-framework MetalKit"
         "-framework AppKit"
 )
endif ()