CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_CONFIGURATION.html)

# CMAKE_TRY_COMPILE_CONFIGURATION

Build configuration used for [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) and [`try_run()`](https://cmake.org/cmake/help/v4.4/command/try_run.html#command:try_run) projects.

Projects built by [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) and [`try_run()`](https://cmake.org/cmake/help/v4.4/command/try_run.html#command:try_run) are built synchronously during the CMake configuration step. Therefore a specific build configuration must be chosen even if the generated build system supports multiple configurations.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
