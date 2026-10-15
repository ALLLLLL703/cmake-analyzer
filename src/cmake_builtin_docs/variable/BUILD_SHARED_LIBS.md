CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/BUILD_SHARED_LIBS.html)

# BUILD_SHARED_LIBS

Tell [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) to default to `SHARED` libraries, instead of `STATIC` libraries, when called with no explicit library type.

Calls to [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) without any explicit library type check the current `BUILD_SHARED_LIBS` variable value. If it is true, then the default library type is `SHARED`. Otherwise, the default is `STATIC`.

For example, the code:

~~~
add_library(example ${sources})
~~~

behaves as if written

~~~
if(BUILD_SHARED_LIBS)
  add_library(example SHARED ${sources})
else()
  add_library(example STATIC ${sources})
endif()
~~~

CMake does not define `BUILD_SHARED_LIBS` by default, but projects often create a cache entry for it using the [`option()`](https://cmake.org/cmake/help/v4.4/command/option.html#command:option) command:

~~~
option(BUILD_SHARED_LIBS "Build using shared libraries" ON)
~~~

This provides a switch that users can control, e.g., with [`cmake -D`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-D). If adding such an option to the project, do so in the top level `CMakeLists.txt` file, before any [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) calls. Note that if bringing external dependencies directly into the build, such as with [`FetchContent`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#module:FetchContent) or a direct call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory), and one of those dependencies has such a call to [`option(BUILD_SHARED_LIBS ...)`](https://cmake.org/cmake/help/v4.4/command/option.html#command:option), the top level project must also call [`option(BUILD_SHARED_LIBS ...)`](https://cmake.org/cmake/help/v4.4/command/option.html#command:option) before bringing in its dependencies. Failure to do so can lead to different behavior between the first and subsequent CMake runs.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
