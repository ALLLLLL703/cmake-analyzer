CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_COMPAT_VERSION.html)

# CMAKE_PROJECT_COMPAT_VERSION

Added in version 4.3.

The compatibility version of the top level project.

This variable holds the compatibility version of the project as specified in the top level CMakeLists.txt file by a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command. In the event that the top level CMakeLists.txt contains multiple [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) calls, the most recently called one from that top level CMakeLists.txt will determine the value that `CMAKE_PROJECT_COMPAT_VERSION` contains. For example, consider the following top level CMakeLists.txt:

~~~
cmake_minimum_required(VERSION 4.3)
project(First VERSION 9.0 COMPAT_VERSION 1.2.3)
project(Second VERSION 9.0 COMPAT_VERSION 3.4.5)
add_subdirectory(sub)
project(Third VERSION 9.0 COMPAT_VERSION 6.7.8)
~~~

And `sub/CMakeLists.txt` with the following contents:

~~~
project(SubProj VERSION 2.0 COMPAT_VERSION 1.0)
message("CMAKE_PROJECT_COMPAT_VERSION = ${CMAKE_PROJECT_COMPAT_VERSION}")
~~~

The most recently seen [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command from the top level CMakeLists.txt would be `project(Second ...)`, so this will print:

~~~
CMAKE_PROJECT_COMPAT_VERSION = 3.4.5
~~~

To obtain the version from the most recent call to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) in the current directory scope or above, see the [`PROJECT_COMPAT_VERSION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_COMPAT_VERSION.html#variable:PROJECT_COMPAT_VERSION) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
