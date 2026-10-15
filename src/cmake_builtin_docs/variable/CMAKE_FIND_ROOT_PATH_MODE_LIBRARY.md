CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH_MODE_LIBRARY.html)

# CMAKE_FIND_ROOT_PATH_MODE_LIBRARY

This variable controls whether the [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) and [`CMAKE_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT.html#variable:CMAKE_SYSROOT) are used by [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library).

If set to `ONLY`, then only the roots in [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) will be searched. If set to `NEVER`, then the roots in [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) will be ignored and only the host system root will be used. If set to `BOTH`, then the host system paths and the paths in [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) will be searched.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
