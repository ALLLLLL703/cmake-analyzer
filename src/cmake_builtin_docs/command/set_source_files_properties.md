CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/set_source_files_properties.html)

# set_source_files_properties

Source files can have properties that affect how they are built.

~~~
set_source_files_properties(<files> ...
                            [DIRECTORY <dirs> ...]
                            [TARGET_DIRECTORY <targets> ...]
                            PROPERTIES <prop1> <value1>
                            [<prop2> <value2>] ...)
~~~

Sets properties associated with source files using a key/value paired list.

Added in version 3.18: By default, source file properties are only visible to targets added in the same directory (`CMakeLists.txt`). Visibility can be set in other directory scopes using one or both of the following options:

**`DIRECTORY <dirs>...`**

The source file properties will be set in each of the `<dirs>` directories' scopes. CMake must already know about each of these source directories, either by having added them through a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) or it being the top level source directory. Relative paths are treated as relative to the current source directory.

**`TARGET_DIRECTORY <targets>...`**

The source file properties will be set in each of the directory scopes where any of the specified `<targets>` were created (the `<targets>` must therefore already exist).

Use [`get_source_file_property()`](https://cmake.org/cmake/help/v4.4/command/get_source_file_property.html#command:get_source_file_property) to get property values. See also the [`set_property(SOURCE)`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property) command.

Note

The [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) source file property may be globally visible. See its documentation for details.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- [`get_source_file_property()`](https://cmake.org/cmake/help/v4.4/command/get_source_file_property.html#command:get_source_file_property)

- [Properties on Source Files](https://cmake.org/cmake/help/v4.4/manual/cmake-properties.7.html#source-file-properties) for the list of properties known to CMake

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
