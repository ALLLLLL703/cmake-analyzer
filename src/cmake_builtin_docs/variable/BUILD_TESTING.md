CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/BUILD_TESTING.html)

# BUILD_TESTING

Control whether the [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module invokes [`enable_testing()`](https://cmake.org/cmake/help/v4.4/command/enable_testing.html#command:enable_testing).

The [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module, when loaded by `include(CTest)`, runs code of the form:

~~~
option(BUILD_TESTING "..." ON)
if (BUILD_TESTING)
   # ...
   enable_testing()
   # ...
endif()
~~~

This creates a `BUILD_TESTING` option that controls whether the [`enable_testing()`](https://cmake.org/cmake/help/v4.4/command/enable_testing.html#command:enable_testing) command is invoked to enable generation of tests to run using [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)). See the [`add_test()`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test) command to create tests.

Note

Call `include(CTest)` in the top-level source directory since [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) expects to find a test file in the top-level build directory.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
