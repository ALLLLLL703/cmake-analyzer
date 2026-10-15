CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOMOC_DEPEND_FILTERS.html)

# CMAKE_AUTOMOC_DEPEND_FILTERS

Added in version 3.9.

Filter definitions used by [`CMAKE_AUTOMOC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOMOC.html#variable:CMAKE_AUTOMOC) to extract file names from source code as additional dependencies for the `moc` file.

This variable is used to initialize the [`AUTOMOC_DEPEND_FILTERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC_DEPEND_FILTERS.html#prop_tgt:AUTOMOC_DEPEND_FILTERS) property on all the targets. See that target property for additional information.

By default it is empty.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
