CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION.html)

# CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION

Added in version 3.24.

Set the test output truncation mode in case a maximum size is configured via the [`CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE.html#variable:CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE) or [`CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE.html#variable:CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE) variables. By default the `tail` of the output will be truncated. Other possible values are `middle` and `head`.

It is initialized by [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)), but may be edited in a `CTestCustom` file. See [`ctest_read_custom_files()`](https://cmake.org/cmake/help/v4.4/command/ctest_read_custom_files.html#command:ctest_read_custom_files) documentation.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
