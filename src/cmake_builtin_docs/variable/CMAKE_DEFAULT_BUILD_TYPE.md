CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEFAULT_BUILD_TYPE.html)

# CMAKE_DEFAULT_BUILD_TYPE

Added in version 3.17.

Specifies the configuration to use by default in a `build.ninja` file in the [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config) generator. If this variable is specified, `build.ninja` uses build rules from `build-<Config>.ninja` by default. All custom commands are executed with this configuration. If the variable is not specified, the first item from [`CMAKE_CONFIGURATION_TYPES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CONFIGURATION_TYPES.html#variable:CMAKE_CONFIGURATION_TYPES) is used instead.

The value of this variable must be one of the items from [`CMAKE_CONFIGURATION_TYPES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CONFIGURATION_TYPES.html#variable:CMAKE_CONFIGURATION_TYPES).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
