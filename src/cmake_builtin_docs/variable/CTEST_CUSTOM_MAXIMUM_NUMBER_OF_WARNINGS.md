CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_NUMBER_OF_WARNINGS.html)

# CTEST_CUSTOM_MAXIMUM_NUMBER_OF_WARNINGS

The maximum number of warnings in a single build step which will be detected. After this, the [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test) command will truncate the output. Defaults to 50.

It is initialized by [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)), but may be edited in a `CTestCustom` file. See [`ctest_read_custom_files()`](https://cmake.org/cmake/help/v4.4/command/ctest_read_custom_files.html#command:ctest_read_custom_files) documentation.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
