CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_XCODE_XCCONFIG.html)

# CMAKE_XCODE_XCCONFIG

Added in version 3.24.

If set, the [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator will register the specified file as a global XCConfig file. For target-level XCConfig files see the [`XCODE_XCCONFIG`](https://cmake.org/cmake/help/v4.4/prop_tgt/XCODE_XCCONFIG.html#prop_tgt:XCODE_XCCONFIG) target property.

This feature is intended to ease migration from native Xcode projects to CMake projects.

Contents of `CMAKE_XCODE_XCCONFIG` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
