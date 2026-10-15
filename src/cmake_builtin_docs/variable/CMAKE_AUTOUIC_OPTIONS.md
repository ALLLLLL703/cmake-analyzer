CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOUIC_OPTIONS.html)

# CMAKE_AUTOUIC_OPTIONS

Additional options for `uic` when using [`CMAKE_AUTOUIC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOUIC.html#variable:CMAKE_AUTOUIC).

This variable is used to initialize the [`AUTOUIC_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOUIC_OPTIONS.html#prop_tgt:AUTOUIC_OPTIONS) property on all the targets. See that target property for additional information.

## EXAMPLE

~~~
# ...
set_property(CMAKE_AUTOUIC_OPTIONS "--no-protection")
# ...
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
