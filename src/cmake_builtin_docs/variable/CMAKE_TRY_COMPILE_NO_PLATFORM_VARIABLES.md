CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_NO_PLATFORM_VARIABLES.html)

# CMAKE_TRY_COMPILE_NO_PLATFORM_VARIABLES

Added in version 3.24.

Set to a true value to tell the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command not to propagate any platform variables into the test project.

The [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command normally passes some CMake variables that configure the platform and toolchain behavior into test projects. See policy [`CMP0137`](https://cmake.org/cmake/help/v4.4/policy/CMP0137.html#policy:CMP0137). This variable may be set to disable that behavior.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
