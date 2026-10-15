CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_NETRC_FILE.html)

# CMAKE_NETRC_FILE

Added in version 3.11.

This variable is used to initialize the `NETRC_FILE` option for the [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download) and [`file(UPLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#upload) commands. See those commands for additional information.

This variable is also used by the [`ExternalProject`](https://cmake.org/cmake/help/v4.4/module/ExternalProject.html#module:ExternalProject) and [`FetchContent`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#module:FetchContent) modules for internal calls to [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download).

The local option takes precedence over this variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
