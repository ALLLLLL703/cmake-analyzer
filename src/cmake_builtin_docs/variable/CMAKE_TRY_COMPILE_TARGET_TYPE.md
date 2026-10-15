CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_TARGET_TYPE.html)

# CMAKE_TRY_COMPILE_TARGET_TYPE

Added in version 3.6.

Type of target generated for [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) calls using the source file signature. Valid values are:

**`EXECUTABLE`**

Use [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) to name the source file in the generated project. This is the default if no value is given.

**`STATIC_LIBRARY`**

Use [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) with the `STATIC` option to name the source file in the generated project. This avoids running the linker and is intended for use with cross-compiling toolchains that cannot link without custom flags or linker scripts.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
