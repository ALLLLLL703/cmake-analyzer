CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_RESOURCE_SPEC_FILE.html)

# CTEST_RESOURCE_SPEC_FILE

Added in version 3.18.

Specify the CTest `ResourceSpecFile` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

This can also be used to specify the resource spec file from a CMake build. If no `RESOURCE_SPEC_FILE` is passed to [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test), and `CTEST_RESOURCE_SPEC_FILE` is not specified in the dashboard script, the value of this variable from the build is used.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
