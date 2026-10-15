CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/set_directory_properties.html)

# set_directory_properties

Set properties of the current directory and subdirectories.

~~~
set_directory_properties(PROPERTIES <prop1> <value1> [<prop2> <value2>] ...)
~~~

Sets properties of the current directory and its subdirectories in key-value pairs.

See also the [`set_property(DIRECTORY)`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property) command.

See [Properties on Directories](https://cmake.org/cmake/help/v4.4/manual/cmake-properties.7.html#directory-properties) for the list of properties known to CMake and their individual documentation for the behavior of each property.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- [`get_directory_property()`](https://cmake.org/cmake/help/v4.4/command/get_directory_property.html#command:get_directory_property)

- the more general [`set_property()`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property) command

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
