CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PROCESSOR.html)

# CMAKE_SYSTEM_PROCESSOR

When not cross-compiling, this variable has the same value as the [`CMAKE_HOST_SYSTEM_PROCESSOR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_PROCESSOR.html#variable:CMAKE_HOST_SYSTEM_PROCESSOR) variable. In many cases, this will correspond to the target architecture for the build, but this is not guaranteed. (E.g. on Windows, the host may be `AMD64` even when using a MSVC `cl` compiler with a 32-bit target.)

When cross-compiling, a [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) should set the `CMAKE_SYSTEM_PROCESSOR` variable to match target architecture that it specifies (via [`CMAKE_<LANG>_COMPILER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER.html#variable:CMAKE_%3CLANG%3E_COMPILER) and perhaps [`CMAKE_<LANG>_COMPILER_TARGET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_TARGET.html#variable:CMAKE_%3CLANG%3E_COMPILER_TARGET)).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
