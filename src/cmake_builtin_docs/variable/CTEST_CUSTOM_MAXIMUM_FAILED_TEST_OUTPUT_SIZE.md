CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE.html)

# CTEST_CUSTOM_MAXIMUM_FAILED_TEST_OUTPUT_SIZE

When saving a failing test's output, this is the maximum size, in bytes, that will be collected by the [`ctest_test()`](https://cmake.org/cmake/help/v4.4/command/ctest_test.html#command:ctest_test) command. Defaults to 307200 (300 KiB). See [`CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION.html#variable:CTEST_CUSTOM_TEST_OUTPUT_TRUNCATION) for possible truncation modes.

If a test's output contains the literal string "CTEST_FULL_OUTPUT", the output will not be truncated and may exceed the maximum size.

It is initialized by [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)), but may be edited in a `CTestCustom` file. See [`ctest_read_custom_files()`](https://cmake.org/cmake/help/v4.4/command/ctest_read_custom_files.html#command:ctest_read_custom_files) documentation.

For controlling the output collection of passing tests, see [`CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE.html#variable:CTEST_CUSTOM_MAXIMUM_PASSED_TEST_OUTPUT_SIZE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
