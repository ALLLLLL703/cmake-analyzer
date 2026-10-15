CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_UNITY_BUILD_RELOCATABLE.html)

# CMAKE_UNITY_BUILD_RELOCATABLE

Added in version 4.0.

This variable is used to initialize the [`UNITY_BUILD_RELOCATABLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/UNITY_BUILD_RELOCATABLE.html#prop_tgt:UNITY_BUILD_RELOCATABLE) property of targets when they are created. Setting it to true causes sources generated for [`CMAKE_UNITY_BUILD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_UNITY_BUILD.html#variable:CMAKE_UNITY_BUILD) to `#include` the original source files using relative paths where possible.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
