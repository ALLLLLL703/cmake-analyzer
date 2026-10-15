CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TLS_VERIFY.html)

# CMAKE_TLS_VERIFY

Specify the default value for the [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download) and [`file(UPLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#upload) commands' `TLS_VERIFY` options. If this variable is not set, the commands check the [`CMAKE_TLS_VERIFY`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TLS_VERIFY.html#envvar:CMAKE_TLS_VERIFY) environment variable. If neither is set, the default is *on*.

Changed in version 3.31: The default is on. Previously, the default was off. Users may set the [`CMAKE_TLS_VERIFY`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TLS_VERIFY.html#envvar:CMAKE_TLS_VERIFY) environment variable to `0` to restore the old default.

This variable is also used by the [`ExternalProject`](https://cmake.org/cmake/help/v4.4/module/ExternalProject.html#module:ExternalProject) and [`FetchContent`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#module:FetchContent) modules for internal calls to [`file(DOWNLOAD)`](https://cmake.org/cmake/help/v4.4/command/file.html#download).

TLS verification can help provide confidence that one is connecting to the desired server. When downloading known content, one should also use file hashes to verify it.

~~~
set(CMAKE_TLS_VERIFY TRUE)
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
