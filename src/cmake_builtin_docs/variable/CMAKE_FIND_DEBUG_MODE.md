CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_DEBUG_MODE.html)

# CMAKE_FIND_DEBUG_MODE

Added in version 3.17.

Print extra find call information for the following commands to standard error:

- [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program)

- [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library)

- [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file)

- [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path)

- [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)

Output is designed for human consumption and not for parsing. Enabling this variable is equivalent to using [`cmake --debug-find`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-debug-find) with the added ability to enable debugging for a subset of find calls.

~~~
set(CMAKE_FIND_DEBUG_MODE TRUE)
find_program(...)
set(CMAKE_FIND_DEBUG_MODE FALSE)
~~~

Default is unset.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
