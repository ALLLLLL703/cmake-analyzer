CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_DESCRIPTION.html)

# CMAKE_PROJECT_DESCRIPTION

Added in version 3.9.

The description of the top level project.

This variable holds the description of the project as specified in the top level CMakeLists.txt file by a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command. In the event that the top level CMakeLists.txt contains multiple [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) calls, the most recently called one from that top level CMakeLists.txt will determine the value that `CMAKE_PROJECT_DESCRIPTION` contains. For example, consider the following top level CMakeLists.txt:

~~~
cmake_minimum_required(VERSION 3.0)
project(First DESCRIPTION "I am First")
project(Second DESCRIPTION "I am Second")
add_subdirectory(sub)
project(Third DESCRIPTION "I am Third")
~~~

And `sub/CMakeLists.txt` with the following contents:

~~~
project(SubProj DESCRIPTION "I am SubProj")
message("CMAKE_PROJECT_DESCRIPTION = ${CMAKE_PROJECT_DESCRIPTION}")
~~~

The most recently seen [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command from the top level CMakeLists.txt would be `project(Second ...)`, so this will print:

~~~
CMAKE_PROJECT_DESCRIPTION = I am Second
~~~

To obtain the description from the most recent call to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) in the current directory scope or above, see the [`PROJECT_DESCRIPTION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_DESCRIPTION.html#variable:PROJECT_DESCRIPTION) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
