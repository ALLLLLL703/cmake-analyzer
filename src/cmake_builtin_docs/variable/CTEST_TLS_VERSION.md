CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_TLS_VERSION.html)

# CTEST_TLS_VERSION

Added in version 3.30.

Specify the CTest `TLSVersion` setting in a [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client) script or in project `CMakeLists.txt` code before including the [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module, or on the **ctest** command line via the [`-D`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-dashboard-D) option. The value is a minimum TLS version allowed when submitting to a dashboard via `https://` URLs.

The value may be one of:

- `1.0`

- `1.1`

- `1.2`

- `1.3`

If `CTEST_TLS_VERSION` is not set, the [`CMAKE_TLS_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TLS_VERSION.html#variable:CMAKE_TLS_VERSION) variable or [`CMAKE_TLS_VERSION`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_TLS_VERSION.html#envvar:CMAKE_TLS_VERSION) environment variable is used instead.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
