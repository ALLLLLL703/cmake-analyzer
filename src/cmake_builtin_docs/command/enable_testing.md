CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/enable_testing.html)

# enable_testing

Enables testing for the current directory and below:

~~~
enable_testing()
~~~

This command should be invoked in the top-level source directory because [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) expects to find a test file in the top-level build directory.

This command is also automatically invoked when the [`CTest`](https://cmake.org/cmake/help/v4.4/module/CTest.html#module:CTest) module is included, except if the [`BUILD_TESTING`](https://cmake.org/cmake/help/v4.4/variable/BUILD_TESTING.html#variable:BUILD_TESTING) option is turned off.

The following restrictions apply to where `enable_testing()` may be called:

- It must be called in file scope, not in a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) call nor inside a [`block()`](https://cmake.org/cmake/help/v4.4/command/block.html#command:block).

## Examples

In the following example, this command is conditionally called depending on how the project is used. For instance, when the Example project is added via the [`FetchContent`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#module:FetchContent) module as a subdirectory of a parent project that defines its own tests, testing for the Example project is disabled.

  `CMakeLists.txt` 

~~~
project(Example)

option(Example_ENABLE_TESTING "Enable testing" ${PROJECT_IS_TOP_LEVEL})

if(Example_ENABLE_TESTING)
  enable_testing()
endif()

# ...

if(Example_ENABLE_TESTING)
  add_test(...)
endif()
~~~

## See Also

- The [`add_test()`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test) command.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
