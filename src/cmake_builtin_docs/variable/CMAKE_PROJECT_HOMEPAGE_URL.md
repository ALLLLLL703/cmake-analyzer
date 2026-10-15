CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_HOMEPAGE_URL.html)

# CMAKE_PROJECT_HOMEPAGE_URL

Added in version 3.12.

The homepage URL of the top level project.

This variable holds the homepage URL of the project as specified in the top level CMakeLists.txt file by a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command. In the event that the top level CMakeLists.txt contains multiple [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) calls, the most recently called one from that top level CMakeLists.txt will determine the value that `CMAKE_PROJECT_HOMEPAGE_URL` contains. For example, consider the following top level CMakeLists.txt:

~~~
cmake_minimum_required(VERSION 3.0)
project(First HOMEPAGE_URL "https://first.example.com")
project(Second HOMEPAGE_URL "https://second.example.com")
add_subdirectory(sub)
project(Third HOMEPAGE_URL "https://third.example.com")
~~~

And `sub/CMakeLists.txt` with the following contents:

~~~
project(SubProj HOMEPAGE_URL "https://subproj.example.com")
message("CMAKE_PROJECT_HOMEPAGE_URL = ${CMAKE_PROJECT_HOMEPAGE_URL}")
~~~

The most recently seen [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command from the top level CMakeLists.txt would be `project(Second ...)`, so this will print:

~~~
CMAKE_PROJECT_HOMEPAGE_URL = https://second.example.com
~~~

To obtain the homepage URL from the most recent call to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) in the current directory scope or above, see the [`PROJECT_HOMEPAGE_URL`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_HOMEPAGE_URL.html#variable:PROJECT_HOMEPAGE_URL) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
