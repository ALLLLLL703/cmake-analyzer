CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TLS_VERSION.html)

# CMAKE_TLS_VERSION

Added in version 3.30.

Specify the default value for the [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download) and [`file(UPLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#upload) commands' `TLS_VERSION` option. If this variable is not set, the commands check the [`CMAKE_TLS_VERSION`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TLS_VERSION.html#envvar:CMAKE_TLS_VERSION) environment variable. If neither is set, the default is TLS 1.2.

Changed in version 3.31: The default is TLS 1.2. Previously, no minimum version was enforced by default.

The value may be one of:

- `1.0`

- `1.1`

- `1.2`

- `1.3`

This variable is also used by the [`ExternalProject`](https://cmake.org/cmake/help/v4.4/module/ExternalProject.html#module:ExternalProject) and [`FetchContent`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#module:FetchContent) modules for internal calls to [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download) and `git clone`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
