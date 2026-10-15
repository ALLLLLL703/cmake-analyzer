CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FASTBUILD_CAPTURE_SYSTEM_ENV.html)

# CMAKE_FASTBUILD_CAPTURE_SYSTEM_ENV

Added in version 4.2.

Controls capturing of the system environment into `fbuild.bff`. Setting it to `OFF` makes the invocation of all tools (compilers and other external processes) hermetic.

Note

Setting this variable to `OFF` can break MSVC toolchains that rely on environment variables such as `INCLUDE` or `LIB` unless these are manually configured elsewhere.

Defaults to `ON`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
