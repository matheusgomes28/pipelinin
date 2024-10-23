cmake_minimum_required(VERSION 3.21)
project(CmakeCppExample LANGUAGES CXX)

# Global C++ standard settings:
#   - Use C++ 17
#   - Require C++ 17 support from compiler
#   - Dissalow compiler-specific extensions
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED On)
set(CMAKE_CXX_EXTENSIONS Off)

include(CTest)
include(dependencies.cmake)

enable_testing()

# Add our library and the main
# executable to the build
add_subdirectory(src/library)
add_subdirectory(src/executable)
add_subdirectory(tests)
