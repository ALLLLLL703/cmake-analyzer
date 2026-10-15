CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MAXIMUM_RECURSION_DEPTH.html)

# CMAKE_MAXIMUM_RECURSION_DEPTH

Added in version 3.14.

Maximum recursion depth for CMake scripts. It is intended to be set on the command line with `-DCMAKE_MAXIMUM_RECURSION_DEPTH=<x>`, or within `CMakeLists.txt` by projects that require a large recursion depth. Projects that set this variable should provide the user with a way to override it. For example:

~~~
# About to perform deeply recursive actions
if(NOT CMAKE_MAXIMUM_RECURSION_DEPTH)
  set(CMAKE_MAXIMUM_RECURSION_DEPTH 2000)
endif()
~~~

If it is not set, or is set to a non-integer value, a sensible default limit is used. If the recursion limit is reached, the script terminates immediately with a fatal error.

Calling any of the following commands increases the recursion depth:

- [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include)

- [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)

- [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory)

- [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile)

- [`ctest_read_custom_files()`](https://cmake.org/cmake/help/v4.4/command/ctest_read_custom_files.html#command:ctest_read_custom_files)

- [`ctest_run_script()`](https://cmake.org/cmake/help/v4.4/command/ctest_run_script.html#command:ctest_run_script) (unless `NEW_PROCESS` is specified)

- User-defined [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function)'s and [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro)'s (note that [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) and [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro) themselves don't increase recursion depth)

- Reading or writing variables that are being watched by a [`variable_watch()`](https://cmake.org/cmake/help/v4.4/command/variable_watch.html#command:variable_watch)

See also the [`CMAKE_MAXIMUM_RECURSION_DEPTH`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_MAXIMUM_RECURSION_DEPTH.html#envvar:CMAKE_MAXIMUM_RECURSION_DEPTH) environment variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
