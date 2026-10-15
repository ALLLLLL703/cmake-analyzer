CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SKIP_TEST_ALL_DEPENDENCY.html)

# CMAKE_SKIP_TEST_ALL_DEPENDENCY

Added in version 3.29.

Control whether the `test` target depends on the `all` target.

If this variable is not defined, or is set to `TRUE`, then the `test` (or `RUN_TESTS`) target does not depend on the `all` (or `ALL_BUILD`) target. When the `test` target is built, e.g., via `make test`, the test process will start immediately, regardless of whether the project has been completely built or not.

If `CMAKE_SKIP_TEST_ALL_DEPENDENCY` is explicitly set to `FALSE`, then the `test` target will depend on the `all` target. When the `test` target is built, e.g., via `make test`, the `all` target will be built first, and then the tests will run.

See also [`CMAKE_SKIP_INSTALL_ALL_DEPENDENCY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SKIP_INSTALL_ALL_DEPENDENCY.html#variable:CMAKE_SKIP_INSTALL_ALL_DEPENDENCY).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
