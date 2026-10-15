CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/remove.html)

# remove

Deprecated since version 3.0: Use the [`list(REMOVE_ITEM)`](https://cmake.org/cmake/help/v4.4/command/list.html#remove-item) command instead.

~~~
remove(VAR VALUE VALUE ...)
~~~

Removes `VALUE` from the variable `VAR`. This is typically used to remove entries from a vector (e.g. semicolon separated list). `VALUE` is expanded.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
