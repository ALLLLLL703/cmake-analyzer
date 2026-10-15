CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/get_directory_property.html)

# get_directory_property

Get a property of `DIRECTORY` scope.

~~~
get_directory_property(<variable> [DIRECTORY <dir>] <prop-name>)
~~~

Stores a property of directory scope in the named `<variable>`.

The `DIRECTORY` argument specifies another directory from which to retrieve the property value instead of the current directory. Relative paths are treated as relative to the current source directory. CMake must already know about the directory, either by having added it through a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) or being the top level directory.

Added in version 3.19: `<dir>` may reference a binary directory.

If the property is not defined for the nominated directory scope, an empty string is returned. In the case of `INHERITED` properties, if the property is not found for the nominated directory scope, the search will chain to a parent scope as described for the [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property) command.

~~~
get_directory_property(<variable> [DIRECTORY <dir>]
                       DEFINITION <var-name>)
~~~

Get a variable definition from a directory. This form is useful to get a variable definition from another directory.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- the more general [`get_property()`](https://cmake.org/cmake/help/v4.4/command/get_property.html#command:get_property) command

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
