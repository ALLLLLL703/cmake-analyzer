CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_read_custom_files.html)

# ctest_read_custom_files

Read CTestCustom files.

~~~
ctest_read_custom_files(<directory>...)
~~~

Read all the `CTestCustom.ctest` or `CTestCustom.cmake` files from the given directory.

By default, invoking [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) without a script will read custom files from the binary directory.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
