CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM.html)

# CMAKE_HOST_SYSTEM

Composite Name of OS CMake is being run on.

This variable is the composite of [`CMAKE_HOST_SYSTEM_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_NAME.html#variable:CMAKE_HOST_SYSTEM_NAME) and [`CMAKE_HOST_SYSTEM_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_VERSION.html#variable:CMAKE_HOST_SYSTEM_VERSION), e.g. `${CMAKE_HOST_SYSTEM_NAME}-${CMAKE_HOST_SYSTEM_VERSION}`. If [`CMAKE_HOST_SYSTEM_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_VERSION.html#variable:CMAKE_HOST_SYSTEM_VERSION) is not set, then this variable is the same as [`CMAKE_HOST_SYSTEM_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_NAME.html#variable:CMAKE_HOST_SYSTEM_NAME).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
