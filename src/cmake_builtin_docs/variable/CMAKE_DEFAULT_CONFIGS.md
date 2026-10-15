CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEFAULT_CONFIGS.html)

# CMAKE_DEFAULT_CONFIGS

Added in version 3.17.

Specifies a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of configurations to build for a target in `build.ninja` if no `:<Config>` suffix is specified in the [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config) generator. If it is set to `all`, all configurations from [`CMAKE_CROSS_CONFIGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSS_CONFIGS.html#variable:CMAKE_CROSS_CONFIGS) are used. If it is not specified, it defaults to [`CMAKE_DEFAULT_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEFAULT_BUILD_TYPE.html#variable:CMAKE_DEFAULT_BUILD_TYPE).

For example, if you set [`CMAKE_DEFAULT_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEFAULT_BUILD_TYPE.html#variable:CMAKE_DEFAULT_BUILD_TYPE) to `Release`, but set `CMAKE_DEFAULT_CONFIGS` to `Debug` or `all`, all `<target>` aliases in `build.ninja` will resolve to `<target>:Debug` or `<target>:all`, but custom commands will still use the `Release` configuration.

The value of this variable must be a subset of [`CMAKE_CROSS_CONFIGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSS_CONFIGS.html#variable:CMAKE_CROSS_CONFIGS) or be the same as [`CMAKE_DEFAULT_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEFAULT_BUILD_TYPE.html#variable:CMAKE_DEFAULT_BUILD_TYPE). It must not be specified if [`CMAKE_DEFAULT_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEFAULT_BUILD_TYPE.html#variable:CMAKE_DEFAULT_BUILD_TYPE) or [`CMAKE_CROSS_CONFIGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSS_CONFIGS.html#variable:CMAKE_CROSS_CONFIGS) is not used.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
