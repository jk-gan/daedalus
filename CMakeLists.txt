cmake_minimum_required(VERSION 3.29)
project(daedalus)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(third_party/SDL3)
add_subdirectory(third_party/spdlog)
#add_subdirectory(third_party/metal-cpp)
#add_subdirectory(src)

add_executable(daedalus
        src/renderer.h
        src/mtk_view_delegate.h
        src/app_delegate.h
        src/app_delegate.cpp
        src/main.cpp
        src/mtk_view_delegate.cpp
        src/renderer.cpp
        src/app.cpp
        src/app.h
)

target_include_directories(daedalus PRIVATE
        SDL3::SDL3
        spdlog::spdlog
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp-extensions
)
target_link_libraries(daedalus PRIVATE
        SDL3::SDL3
        spdlog::spdlog
        "-framework Metal"
        "-framework Foundation"
        "-framework QuartzCore"
        "-framework MetalKit"
        "-framework AppKit"
)