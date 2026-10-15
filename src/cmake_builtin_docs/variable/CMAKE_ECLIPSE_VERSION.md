CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ECLIPSE_VERSION.html)

# CMAKE_ECLIPSE_VERSION

Added in version 3.6.

This cache variable is used by the Eclipse project generator. See [`cmake-generators(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#manual:cmake-generators(7)).

When using the Eclipse project generator, CMake tries to find the Eclipse executable and detect the version of it. Depending on the version it finds, some features are enabled or disabled. If CMake doesn't find Eclipse, it assumes the oldest supported version, Eclipse Callisto (3.2).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
