CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTORCC_OPTIONS.html)

# CMAKE_AUTORCC_OPTIONS

Additional options for `rcc` when using [`CMAKE_AUTORCC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTORCC.html#variable:CMAKE_AUTORCC).

This variable is used to initialize the [`AUTORCC_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTORCC_OPTIONS.html#prop_tgt:AUTORCC_OPTIONS) property on all the targets. See that target property for additional information.

## EXAMPLE

~~~
# ...
set(CMAKE_AUTORCC_OPTIONS "--compress;9")
# ...
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
