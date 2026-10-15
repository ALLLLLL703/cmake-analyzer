CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/PROJECT_IS_TOP_LEVEL.html)

# PROJECT_IS_TOP_LEVEL

Added in version 3.21.

A boolean variable indicating whether the most recently called [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command in the current scope or above was in the top level `CMakeLists.txt` file.

Some modules should only be included as part of the top level `CMakeLists.txt` file to not cause unintended side effects in the build tree, and this variable can be used to conditionally execute such code. For example, consider the [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module, which creates targets and options:

~~~
project(MyProject)
...
if(PROJECT_IS_TOP_LEVEL)
  include(CTest)
endif()
~~~

The variable value will be true in:

- the top-level directory of the project

- the top-level directory of an external project added by [`ExternalProject`](https://cmake.org/cmake/help/v4.4/module/ExternalProject.html#module:ExternalProject)

- a directory added by [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) that does not also contain a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

- a directory added by [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable), if the fetched content does not contain a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

The variable value will be false in:

- a directory added by [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) that also contains a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

- a directory added by [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable), if the fetched content contains a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
