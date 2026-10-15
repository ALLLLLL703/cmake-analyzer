CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_PATH.html)

# CMAKE_MODULE_PATH

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of directories, represented using forward slashes, specifying a search path for CMake modules to be loaded by the [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include) or [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) commands before checking the default modules that come with CMake. By default it is empty. It is intended to be set by the project.

It's fairly common for a project to have a directory containing various `*.cmake` files to assist in development. Adding the directory to the [`CMAKE_MODULE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_PATH.html#variable:CMAKE_MODULE_PATH) simplifies loading them. For example, a project's top-level `CMakeLists.txt` file may contain:

~~~
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

include(Foo) # Loads ${CMAKE_CURRENT_SOURCE_DIR}/cmake/Foo.cmake

find_package(Bar) # Loads ${CMAKE_CURRENT_SOURCE_DIR}/cmake/FindBar.cmake
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
