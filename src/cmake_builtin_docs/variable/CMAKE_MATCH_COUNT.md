CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MATCH_COUNT.html)

# CMAKE_MATCH_COUNT

Added in version 3.2.

The number of matches with the last regular expression.

When a regular expression match is used, CMake fills in [`CMAKE_MATCH_<n>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MATCH_n.html#variable:CMAKE_MATCH_%3Cn%3E) variables with the match contents. The `CMAKE_MATCH_COUNT` variable holds the number of match expressions when these are filled.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
