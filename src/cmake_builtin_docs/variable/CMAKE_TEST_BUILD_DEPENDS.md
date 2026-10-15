CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TEST_BUILD_DEPENDS.html)

# CMAKE_TEST_BUILD_DEPENDS

Added in version 4.4.

Enable `test_prep/<name>` build targets for tests added by [`add_test()`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test) when using the [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators).

When this variable is set to a true value, CMake generates a `test_prep/<name>` target for each eligible test and a `test_prep/all` target for all such tests. Building these targets ensures the test executable, targets referenced by test command generator expressions, and explicit `BUILD_DEPENDS` entries are up-to-date before the test runs.

Tests whose names are not valid target names are excluded from this behavior. If multiple tests in different directories share the same name, their dependencies are merged into a single `test_prep/<name>` target.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
