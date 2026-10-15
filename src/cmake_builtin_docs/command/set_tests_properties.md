CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/set_tests_properties.html)

# set_tests_properties

Set a property of the tests.

~~~
set_tests_properties(<tests>...
                     [DIRECTORY <dir>]
                     PROPERTIES <prop1> <value1>
                     [<prop2> <value2>]...)
~~~

Sets a property for the tests. If the test is not found, CMake will report an error.

Test property values may be specified using [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) for tests created by the [`add_test(NAME)`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test) signature.

Added in version 3.28: Visibility can be set in other directory scopes using the following option:

**`DIRECTORY <dir>`**

The test properties will be set in the `<dir>` directory's scope. CMake must already know about this directory, either by having added it through a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) or it being the top level source directory. Relative paths are treated as relative to the current source directory. `<dir>` may reference a binary directory.

## See Also

- [`add_test()`](https://cmake.org/cmake/help/v4.4/command/add_test.html#command:add_test)

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- the more general [`set_property()`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property) command

- [Properties on Tests](https://cmake.org/cmake/help/v4.4/manual/cmake-properties.7.html#test-properties) for the list of properties known to CMake

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
