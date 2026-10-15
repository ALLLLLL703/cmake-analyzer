CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_APPLE_SILICON_PROCESSOR.html)

# CMAKE_APPLE_SILICON_PROCESSOR

Added in version 3.19.2.

On Apple Silicon hosts running macOS, set this variable to tell CMake what architecture to use for [`CMAKE_HOST_SYSTEM_PROCESSOR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_PROCESSOR.html#variable:CMAKE_HOST_SYSTEM_PROCESSOR). The value must be either `arm64` or `x86_64`.

The value of this variable should never be modified by project code. It is meant to be set as a cache entry provided by the user, e.g. via `-DCMAKE_APPLE_SILICON_PROCESSOR=...`.

See also the [`CMAKE_APPLE_SILICON_PROCESSOR`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_APPLE_SILICON_PROCESSOR.html#envvar:CMAKE_APPLE_SILICON_PROCESSOR) environment variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
