CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SCRIPT_MODE_FILE.html)

# CMAKE_SCRIPT_MODE_FILE

Full path to the [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P) script file currently being processed.

When run in [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P) script mode, CMake sets this variable to the full path of the script file. When run to configure a `CMakeLists.txt` file, this variable is not set.

## See Also

- The [`CMAKE_ROLE`](https://cmake.org/cmake/help/v4.4/prop_gbl/CMAKE_ROLE.html#prop_gbl:CMAKE_ROLE) global property provides the current running mode.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
