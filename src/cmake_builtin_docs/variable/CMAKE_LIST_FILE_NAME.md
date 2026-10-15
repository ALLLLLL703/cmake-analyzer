CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LIST_FILE_NAME.html)

# CMAKE_LIST_FILE_NAME

Added in version 4.0.

The name of the CMake project files. This determines the top-level file processed when CMake is configured, and the file processed by [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory).

By default, this is `CMakeLists.txt`. If set to anything else, `CMakeLists.txt` will be used as a fallback whenever the specified file cannot be found within a project subdirectory.

This variable reports the value set via the [`cmake --project-file`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-project-file) option. The value of this variable should never be set directly by projects or users.

Warning

The use of alternate project file names is intended for temporary use by developers during an incremental transition and not for publication of a final product. CMake will always emit a warning when the project file is anything other than `CMakeLists.txt`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
