CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/get_cmake_property.html)

# get_cmake_property

Get a global property of the CMake instance.

~~~
get_cmake_property(<variable> <property>)
~~~

Gets a global property from the CMake instance. The value of the `<property>` is stored in the specified `<variable>`. If the property is not found, `<variable>` will be set to `NOTFOUND`. See the [`cmake-properties(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-properties.7.html#manual:cmake-properties(7)) manual for available properties.

In addition to global properties, this command (for historical reasons) also supports the [`VARIABLES`](https://cmake.org/cmake/help/v4.4/prop_dir/VARIABLES.html#prop_dir:VARIABLES) and [`MACROS`](https://cmake.org/cmake/help/v4.4/prop_dir/MACROS.html#prop_dir:MACROS) directory properties. It also supports a special `COMPONENTS` global property that lists the components given to the [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) command.

## See Also

- the [`get_property()`](https://cmake.org/cmake/help/v4.4/command/get_property.html#command:get_property) command `GLOBAL` option

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
