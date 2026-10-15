CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSS_CONFIGS.html)

# CMAKE_CROSS_CONFIGS

Added in version 3.17.

Specifies a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of configurations available from all `build-<Config>.ninja` files in the [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config) generator. This variable activates cross-config mode. Targets from each config specified in this variable can be built from any `build-<Config>.ninja` file. Custom commands will use the configuration native to `build-<Config>.ninja`. If it is set to `all`, all configurations from [`CMAKE_CONFIGURATION_TYPES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CONFIGURATION_TYPES.html#variable:CMAKE_CONFIGURATION_TYPES) are cross-configs. If it is not specified, or empty, each `build-<Config>.ninja` file will only contain build rules for its own configuration.

The value of this variable must be a subset of [`CMAKE_CONFIGURATION_TYPES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CONFIGURATION_TYPES.html#variable:CMAKE_CONFIGURATION_TYPES).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
