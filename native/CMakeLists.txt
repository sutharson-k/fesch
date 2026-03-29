cmake_minimum_required(VERSION 3.15)
project(agent_native)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Source files
set(SOURCES
    src/pc_control.cpp
)

# Header files
set(HEADERS
    src/bridge.h
)

# Create shared library
add_library(agent_native SHARED ${SOURCES} ${HEADERS})

# Platform-specific settings
if(WIN32)
    target_link_libraries(agent_native PRIVATE user32 shell32 psapi)
    target_compile_definitions(agent_native PRIVATE _WIN32)
elseif(APPLE)
    find_library(COREFOUNDATION CoreFoundation)
    find_library(APPLICATIONSERVICES ApplicationServices)
    target_link_libraries(agent_native PRIVATE ${COREFOUNDATION} ${APPLICATIONSERVICES})
else()
    # Linux
    find_package(X11 REQUIRED)
    target_link_libraries(agent_native PRIVATE ${X11_LIBRARIES} Xtst)
    target_include_directories(agent_native PRIVATE ${X11_INCLUDE_DIRS})
endif()

# Output directory
set_target_properties(agent_native PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/build"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/build"
)
