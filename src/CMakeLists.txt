add_executable(daedalus main.cpp)

target_include_directories(daedalus PRIVATE Vulkan::Vulkan SDL3::SDL3)
target_link_libraries(daedalus PRIVATE Vulkan::Vulkan SDL3::SDL3)
