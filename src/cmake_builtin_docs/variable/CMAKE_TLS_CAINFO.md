CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TLS_CAINFO.html)

# CMAKE_TLS_CAINFO

Specify the default value for the [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download) and [`file(UPLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#upload) commands' `TLS_CAINFO` options. It is unset by default.

This variable is also used by the [`ExternalProject`](https://cmake.org/cmake/help/v4.4/module/ExternalProject.html#module:ExternalProject) and [`FetchContent`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#module:FetchContent) modules for internal calls to [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
