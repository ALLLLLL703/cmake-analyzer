CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_MEMORYCHECK_SANITIZER_OPTIONS.html)

# CTEST_MEMORYCHECK_SANITIZER_OPTIONS

Added in version 3.1.

Specify the CTest `MemoryCheckSanitizerOptions` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

CTest prepends correct sanitizer options `*_OPTIONS` environment variable to executed command. CTests adds its own `log_path` to sanitizer options, don't provide your own `log_path`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
