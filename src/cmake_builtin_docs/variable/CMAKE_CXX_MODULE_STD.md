CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CXX_MODULE_STD.html)

# CMAKE_CXX_MODULE_STD

Added in version 3.30.

Whether to add utility targets as dependencies to targets with at least `cxx_std_23` or not.

Note

This setting is meaningful only when experimental support for `import
std;` has been enabled by the `CMAKE_EXPERIMENTAL_CXX_IMPORT_STD` gate.

This variable is used to initialize the [`CXX_MODULE_STD`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_MODULE_STD.html#prop_tgt:CXX_MODULE_STD) property on all targets. See that target property for additional information.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
