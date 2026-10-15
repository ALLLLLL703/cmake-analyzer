CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/remove_definitions.html)

# remove_definitions

Removes compile definitions added by [`add_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/add_compile_definitions.html#command:add_compile_definitions), or [`add_definitions()`](https://cmake.org/cmake/help/v4.4/command/add_definitions.html#command:add_definitions):

~~~
remove_definitions([<definitions>...])
~~~

The arguments are:

**`<definitions>...`**

Zero or more compile definitions.

This command can be also used to remove any flags added by [`add_definitions()`](https://cmake.org/cmake/help/v4.4/command/add_definitions.html#command:add_definitions), but it is intended to remove preprocessor definitions passed with `-D`, or `/D`.

## Examples

In the following example targets of the current directory scope will have only `BAZ` and `QUUX` compile definitions:

~~~
add_compile_definitions(FOO BAR BAZ -DQUUX)

# ...

remove_definitions(-DFOO -DBAR)
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
