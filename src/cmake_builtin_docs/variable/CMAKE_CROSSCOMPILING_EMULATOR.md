CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSSCOMPILING_EMULATOR.html)

# CMAKE_CROSSCOMPILING_EMULATOR

Added in version 3.3.

This variable is only used when [`CMAKE_CROSSCOMPILING`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSSCOMPILING.html#variable:CMAKE_CROSSCOMPILING) is on. It should point to a command on the host system that can run executable built for the target system.

Added in version 3.15: If this variable contains a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists), then the first value is the command and remaining values are its arguments.

Added in version 3.28: This variable can be initialized via an [`CMAKE_CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_CROSSCOMPILING_EMULATOR.html#envvar:CMAKE_CROSSCOMPILING_EMULATOR) environment variable.

The command will be used to run [`try_run()`](https://cmake.org/cmake/help/v4.4/command/try_run.html#command:try_run) generated executables, which avoids manual population of the `TryRunResults.cmake` file.

This variable is also used as the default value for the [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) target property of executables. However, while [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) are supported by the target property (since CMake 3.29), they are *not* supported by this variable's [`try_run()`](https://cmake.org/cmake/help/v4.4/command/try_run.html#command:try_run) functionality.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
