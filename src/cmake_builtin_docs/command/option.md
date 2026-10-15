CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/option.html)

# option

Provide a boolean option that the user can optionally select.

~~~
option(<variable> "<help_text>" [value])
~~~

If no initial `<value>` is provided, boolean `OFF` is the default value. If `<variable>` is already set as a normal or cache variable, then the command does nothing (see policy [`CMP0077`](https://cmake.org/cmake/help/v4.4/policy/CMP0077.html#policy:CMP0077)).

In CMake project mode, a boolean cache variable is created with the option value. In CMake script mode, a boolean variable is set with the option value.

## See Also

- The [`CMakeDependentOption`](https://cmake.org/cmake/help/v4.4/module/CMakeDependentOption.html#module:CMakeDependentOption) module to specify boolean options that depend on the values of other options or a set of conditions.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
