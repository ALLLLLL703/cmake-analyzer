CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html)

# CTEST_PRESET

Added in version 4.4.

Specify a preset name to use for all relevant [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) steps in a dashboard script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

This variable sets the `PRESET` option for [`ctest_configure()`](https://cmake.org/cmake/help/v4.4/command/ctest_configure.html#command:ctest_configure), [`ctest_build()`](https://cmake.org/cmake/help/v4.4/command/ctest_build.html#command:ctest_build), [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test), and [`ctest_memcheck()`](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html#command:ctest_memcheck) when those commands are not given an explicit `PRESET` argument and the corresponding command-specific variable ([`CTEST_CONFIGURE_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CONFIGURE_PRESET.html#variable:CTEST_CONFIGURE_PRESET), [`CTEST_BUILD_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BUILD_PRESET.html#variable:CTEST_BUILD_PRESET), or [`CTEST_TEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_TEST_PRESET.html#variable:CTEST_TEST_PRESET)) is also unset.

For [`ctest_configure()`](https://cmake.org/cmake/help/v4.4/command/ctest_configure.html#command:ctest_configure), an error is raised if no configure preset with this name exists. For [`ctest_build()`](https://cmake.org/cmake/help/v4.4/command/ctest_build.html#command:ctest_build), [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test), and [`ctest_memcheck()`](https://cmake.org/cmake/help/v4.4/command/ctest_memcheck.html#command:ctest_memcheck), a warning is emitted and the variable is ignored if no preset of the matching type exists with this name.

See also [`CTEST_PRESETS_FILE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESETS_FILE.html#variable:CTEST_PRESETS_FILE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
