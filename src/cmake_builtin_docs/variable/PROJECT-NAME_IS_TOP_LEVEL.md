CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_IS_TOP_LEVEL.html)

# \<PROJECT-NAME\>_IS_TOP_LEVEL

Added in version 3.21.

A boolean variable indicating whether the named project was called in a top level `CMakeLists.txt` file.

To obtain the value from the most recent call to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) in the current directory scope or above, see the [`PROJECT_IS_TOP_LEVEL`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_IS_TOP_LEVEL.html#variable:PROJECT_IS_TOP_LEVEL) variable.

The variable value will be true in:

- the top-level directory of the project

- the top-level directory of an external project added by [`ExternalProject`](https://cmake.org/cmake/help/v4.4/module/ExternalProject.html#module:ExternalProject)

- a directory added by [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) that does not also contain a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

- a directory added by [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable), if the fetched content does not contain a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

The variable value will be false in:

- a directory added by [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) that also contains a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

- a directory added by [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable), if the fetched content contains a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
