CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/get_source_file_property.html)

# get_source_file_property

Get a property for a source file.

~~~
get_source_file_property(<variable> <file>
                         [DIRECTORY <dir> | TARGET_DIRECTORY <target>]
                         <property>)
~~~

Gets a property from a source file. The value of the property is stored in the specified `<variable>`. If the `<file>` is not a source file, or the source property is not found, `<variable>` will be set to `NOTFOUND`. If the source property was defined to be an `INHERITED` property (see [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)), the search will include the relevant parent scopes, as described for the [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property) command.

By default, the source file's property will be read from the current source directory's scope.

Added in version 3.18: Directory scope can be overridden with one of the following sub-options:

**`DIRECTORY <dir>`**

The source file property will be read from the `<dir>` directory's scope. CMake must already know about that source directory, either by having added it through a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) or `<dir>` being the top level source directory. Relative paths are treated as relative to the current source directory.

**`TARGET_DIRECTORY <target>`**

The source file property will be read from the directory scope in which `<target>` was created (`<target>` must therefore already exist).

Use [`set_source_files_properties()`](https://cmake.org/cmake/help/v4.4/command/set_source_files_properties.html#command:set_source_files_properties) to set property values. Source file properties usually control how the file is built. One property that is always there is [`LOCATION`](https://cmake.org/cmake/help/v4.4/prop_sf/LOCATION.html#prop_sf:LOCATION).

Note

The [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) source file property may be globally visible. See its documentation for details.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- the more general [`get_property()`](https://cmake.org/cmake/help/v4.4/command/get_property.html#command:get_property) command

- [`set_source_files_properties()`](https://cmake.org/cmake/help/v4.4/command/set_source_files_properties.html#command:set_source_files_properties)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
