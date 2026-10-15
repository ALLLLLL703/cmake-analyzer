CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_TLS_VERIFY.html)

# CTEST_TLS_VERIFY

Added in version 3.30.

Specify the CTest `TLSVerify` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script or in project `CMakeLists.txt` code before including the [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option. The value is a boolean indicating whether to verify the server certificate when submitting to a dashboard via `https://` URLs.

If `CTEST_TLS_VERIFY` is not set, the [`CMAKE_TLS_VERIFY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TLS_VERIFY.html#variable:CMAKE_TLS_VERIFY) variable or [`CMAKE_TLS_VERIFY`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TLS_VERIFY.html#envvar:CMAKE_TLS_VERIFY) environment variable is used instead. If neither is set, the default is *on*.

Changed in version 3.31: The default is on. Previously, the default was off. Users may set the [`CMAKE_TLS_VERIFY`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TLS_VERIFY.html#envvar:CMAKE_TLS_VERIFY) environment variable to `0` to restore the old default.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
