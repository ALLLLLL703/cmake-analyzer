CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/get_test_property.html)

# get_test_property

Get a property of the test.

~~~
get_test_property(<test> <property> [DIRECTORY <dir>] <variable>)
~~~

Get a property from the test. The value of the property is stored in the specified `<variable>`. If the `<test>` is not defined, or the test property is not found, `<variable>` will be set to `NOTFOUND`. If the test property was defined to be an `INHERITED` property (see [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)), the search will include the relevant parent scopes, as described for the [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property) command.

For a list of standard properties you can type [`cmake --help-property-list`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-help-property-list).

Added in version 3.28: Directory scope can be overridden with the following sub-option:

**`DIRECTORY <dir>`**

The test property will be read from the `<dir>` directory's scope. CMake must already know about that source directory, either by having added it through a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) or `<dir>` being the top level source directory. Relative paths are treated as relative to the current source directory. `<dir>` may reference a binary directory.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- the more general [`get_property()`](https://cmake.org/cmake/help/v4.4/command/get_property.html#command:get_property) command

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
