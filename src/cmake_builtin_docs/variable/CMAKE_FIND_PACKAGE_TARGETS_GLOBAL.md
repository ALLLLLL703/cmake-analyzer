CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_TARGETS_GLOBAL.html)

# CMAKE_FIND_PACKAGE_TARGETS_GLOBAL

Added in version 3.24.

Setting to `TRUE` promotes all [`IMPORTED`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED.html#prop_tgt:IMPORTED) targets discovered by [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) to a `GLOBAL` scope.

Setting this to `TRUE` is akin to specifying `GLOBAL` as an argument to [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package). Default value is `OFF`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
