CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ARGV0.html)

# CMAKE_ARGV0

Command line argument passed to CMake in script mode.

When run in [-P](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#script-processing-mode) script mode, CMake sets this variable to the first command line argument. It then also sets `CMAKE_ARGV1`, `CMAKE_ARGV2`, ... and so on, up to the number of command line arguments given. See also [`CMAKE_ARGC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ARGC.html#variable:CMAKE_ARGC).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
