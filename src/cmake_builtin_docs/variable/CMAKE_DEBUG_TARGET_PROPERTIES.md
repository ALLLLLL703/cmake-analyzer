CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEBUG_TARGET_PROPERTIES.html)

# CMAKE_DEBUG_TARGET_PROPERTIES

Enables tracing output for target properties.

This variable can be populated with a list of properties to generate debug output for when evaluating target properties. Currently it can only be used when evaluating:

- [`AUTOUIC_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOUIC_OPTIONS.html#prop_tgt:AUTOUIC_OPTIONS)

- [`COMPILE_DEFINITIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_DEFINITIONS.html#prop_tgt:COMPILE_DEFINITIONS)

- [`COMPILE_FEATURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_FEATURES.html#prop_tgt:COMPILE_FEATURES)

- [`COMPILE_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_OPTIONS.html#prop_tgt:COMPILE_OPTIONS)

- [`INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INCLUDE_DIRECTORIES.html#prop_tgt:INCLUDE_DIRECTORIES)

- [`LINK_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_DIRECTORIES.html#prop_tgt:LINK_DIRECTORIES)

- [`LINK_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_OPTIONS.html#prop_tgt:LINK_OPTIONS)

- [`POSITION_INDEPENDENT_CODE`](https://cmake.org/cmake/help/v4.4/prop_tgt/POSITION_INDEPENDENT_CODE.html#prop_tgt:POSITION_INDEPENDENT_CODE)

- [`SOURCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCES.html#prop_tgt:SOURCES)

target properties and any other property listed in [`COMPATIBLE_INTERFACE_STRING`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPATIBLE_INTERFACE_STRING.html#prop_tgt:COMPATIBLE_INTERFACE_STRING) and other `COMPATIBLE_INTERFACE_` properties. It outputs an origin for each entry in the target property. Default is unset.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
