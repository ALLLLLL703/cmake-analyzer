CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/link_libraries.html)

# link_libraries

Link libraries to all targets added later.

~~~
link_libraries([item1 [item2 [...]]]
               [[debug|optimized|general] <item>] ...)
~~~

Specify libraries or flags to use when linking any targets created later in the current directory or below by commands such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library). See the [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) command for meaning of arguments.

Note

The [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) command should be preferred whenever possible. Library dependencies are chained automatically, so directory-wide specification of link libraries is rarely needed.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
