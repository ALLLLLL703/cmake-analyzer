CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_CHANGE_ID.html)

# CTEST_CHANGE_ID

Added in version 3.4.

Specify the CTest `ChangeId` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option.

This setting allows CTest to pass arbitrary information about this build up to CDash. One use of this feature is to allow CDash to post comments on your pull request if anything goes wrong with your build.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
