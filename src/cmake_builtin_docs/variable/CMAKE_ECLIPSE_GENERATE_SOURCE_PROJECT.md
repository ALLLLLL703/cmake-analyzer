CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ECLIPSE_GENERATE_SOURCE_PROJECT.html)

# CMAKE_ECLIPSE_GENERATE_SOURCE_PROJECT

Added in version 3.6.

This cache variable is used by the Eclipse project generator. See [`cmake-generators(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#manual:cmake-generators(7)).

If this variable is set to TRUE, the Eclipse project generator will generate an Eclipse project in [`CMAKE_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SOURCE_DIR.html#variable:CMAKE_SOURCE_DIR) . This project can then be used in Eclipse e.g. for the version control functionality. `CMAKE_ECLIPSE_GENERATE_SOURCE_PROJECT` defaults to `FALSE`; so nothing is written into the source directory.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
