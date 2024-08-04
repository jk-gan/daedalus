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
add_subdirectory(third_party/glm)

# Add source files
file(GLOB_RECURSE PROJECT_HEADERS CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/src/*.h)
file(GLOB_RECURSE PROJECT_SOURCES CONFIGURE_DEPENDS ${CMAKE_SOURCE_DIR}/src/*.cpp)

# Add shader files
set(METAL_SHADER_DIR ${CMAKE_SOURCE_DIR}/assets/shaders)
set(COMPILED_SHADER_DIR ${CMAKE_BINARY_DIR}/compiled_shaders)
set(COMPILED_SHADER_DIR ${CMAKE_BINARY_DIR}/assets/shaders/compiled)

file(GLOB_RECURSE SHADERS CONFIGURE_DEPENDS ${METAL_SHADER_DIR}/*.metal)

# Compile shaders
foreach (SHADER ${SHADERS})
 get_filename_component(SHADER_NAME ${SHADER} NAME_WE)
 set(COMPILED_SHADER ${COMPILED_SHADER_DIR}/${SHADER_NAME}.metallib)

 add_custom_command(
         OUTPUT ${COMPILED_SHADER}
         COMMAND xcrun -sdk macosx metal -c ${SHADER} -o ${COMPILED_SHADER_DIR}/${SHADER_NAME}.air
         COMMAND xcrun -sdk macosx metallib ${COMPILED_SHADER_DIR}/${SHADER_NAME}.air -o ${COMPILED_SHADER}
         DEPENDS ${SHADER}
         COMMENT "Compiling Metal shader ${SHADER_NAME}"
 )

 list(APPEND COMPILED_SHADERS ${COMPILED_SHADER})
endforeach ()

add_executable(daedalus ${PROJECT_HEADERS} ${PROJECT_SOURCES} ${COMPILED_SHADERS}
        assets/shaders/definitions.h)

target_include_directories(daedalus PRIVATE
        SDL3::SDL3
        spdlog::spdlog
        glm::glm
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp
        ${PROJECT_SOURCE_DIR}/third_party/metal-cpp-extensions
)
if (IOS)
 target_link_libraries(daedalus PRIVATE
         SDL3::SDL3
         spdlog::spdlog
         glm::glm
         "-framework Metal"
         "-framework Foundation"
         "-framework QuartzCore"
         "-framework MetalKit"
 )
else ()
 target_link_libraries(daedalus PRIVATE
         SDL3::SDL3
         spdlog::spdlog
         glm::glm
         "-framework Metal"
         "-framework Foundation"
         "-framework QuartzCore"
         "-framework MetalKit"
         "-framework AppKit"
 )
endif ()