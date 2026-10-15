CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CXX_COMPILER_IMPORT_STD.html)

# CMAKE_CXX_COMPILER_IMPORT_STD

Added in version 3.30.

A list of C++ standard levels for which `import std` support exists for the current C++ toolchain. Support for C++\<NN\> may be detected using a `<NN> IN_LIST CMAKE_CXX_COMPILER_IMPORT_STD` predicate with the [`if()`](https://cmake.org/cmake/help/v4.4/command/if.html#command:if) command.

Note

This variable is meaningful only when experimental support for `import
std;` has been enabled by the `CMAKE_EXPERIMENTAL_CXX_IMPORT_STD` gate.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
