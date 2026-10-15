CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TEST_LAUNCHER.html)

# CMAKE_TEST_LAUNCHER

Added in version 3.29.

This variable is used to initialize the [`TEST_LAUNCHER`](https://cmake.org/cmake/help/v4.4/prop_tgt/TEST_LAUNCHER.html#prop_tgt:TEST_LAUNCHER) target property of executable targets as they are created. It is used to specify a launcher for running tests, added by the [`add_test()`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test) command, that run an executable target.

If this variable contains a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists), then the first value is the command and remaining values are its arguments.

This variable can be initialized via an [`CMAKE_TEST_LAUNCHER`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TEST_LAUNCHER.html#envvar:CMAKE_TEST_LAUNCHER) environment variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
