CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CTEST_CUSTOM_ERROR_EXCEPTION.html)

# CTEST_CUSTOM_ERROR_EXCEPTION

A list of regular expressions which will be used to exclude when detecting error messages in build outputs by the [`ctest_build()`](https://cmake.org/cmake/help/v4.4/command/ctest_build.html#command:ctest_build) command.

It is initialized by [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)), but may be edited in a `CTestCustom` file. See [`ctest_read_custom_files()`](https://cmake.org/cmake/help/v4.4/command/ctest_read_custom_files.html#command:ctest_read_custom_files) documentation.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
