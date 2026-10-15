CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_NAME.html)

# CMAKE_PROJECT_NAME

The name of the top level project.

This variable holds the name of the project as specified in the top level CMakeLists.txt file by a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command. In the event that the top level CMakeLists.txt contains multiple [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) calls, the most recently called one from that top level CMakeLists.txt will determine the name that `CMAKE_PROJECT_NAME` contains. For example, consider the following top level CMakeLists.txt:

~~~
cmake_minimum_required(VERSION 3.0)
project(First)
project(Second)
add_subdirectory(sub)
project(Third)
~~~

And `sub/CMakeLists.txt` with the following contents:

~~~
project(SubProj)
message("CMAKE_PROJECT_NAME = ${CMAKE_PROJECT_NAME}")
~~~

The most recently seen [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command from the top level CMakeLists.txt would be `project(Second)`, so this will print:

~~~
CMAKE_PROJECT_NAME = Second
~~~

To obtain the name from the most recent call to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) in the current directory scope or above, see the [`PROJECT_NAME`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_NAME.html#variable:PROJECT_NAME) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
