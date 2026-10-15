CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_NIGHTLY_START_TIME.html)

# CTEST_NIGHTLY_START_TIME

Added in version 3.1.

Specify the CTest `NightlyStartTime` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

Note that this variable must always be set for a nightly build in a dashboard script. It is needed so that nightly builds can be properly grouped together in CDash.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
