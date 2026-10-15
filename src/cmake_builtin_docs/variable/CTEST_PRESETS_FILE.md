CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESETS_FILE.html)

# CTEST_PRESETS_FILE

Added in version 4.4.

Specify the `PRESETS_FILE` option for all relevant [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) step commands in a dashboard script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

This variable sets the `PRESETS_FILE` option for [`ctest_configure()`](https://cmake.org/cmake/help/v4.4/command/ctest_configure.html#command:ctest_configure), [`ctest_build()`](https://cmake.org/cmake/help/v4.4/command/ctest_build.html#command:ctest_build), [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test), and [`ctest_memcheck()`](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html#command:ctest_memcheck) when those commands are not given an explicit `PRESETS_FILE` argument.

See also [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET), [`CTEST_CONFIGURE_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CONFIGURE_PRESET.html#variable:CTEST_CONFIGURE_PRESET), [`CTEST_BUILD_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BUILD_PRESET.html#variable:CTEST_BUILD_PRESET), and [`CTEST_TEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_TEST_PRESET.html#variable:CTEST_TEST_PRESET).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
