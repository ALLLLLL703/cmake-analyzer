CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html)

# CMAKE_FIND_ROOT_PATH

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of root paths to search on the filesystem.

This variable is most useful when cross-compiling. CMake uses the paths in this list as alternative roots to find filesystem items with [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package), [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library) etc.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
