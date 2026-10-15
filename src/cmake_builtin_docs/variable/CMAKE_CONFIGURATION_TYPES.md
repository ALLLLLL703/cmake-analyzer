CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CONFIGURATION_TYPES.html)

# CMAKE_CONFIGURATION_TYPES

Specifies the available build types (configurations) on multi-config generators (e.g. [Visual Studio](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators), [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode), or [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config)) as a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists). Typical entries include `Debug`, `Release`, `RelWithDebInfo` and `MinSizeRel`, but custom build types can also be defined.

This variable is initialized by the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command called in a project when a new build tree is first created. If the [`CMAKE_CONFIGURATION_TYPES`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_CONFIGURATION_TYPES.html#envvar:CMAKE_CONFIGURATION_TYPES) environment variable is set, its value is used. Otherwise, the default value is generator-specific.

Depending on the situation, the values in this variable may be treated case-sensitively or case-insensitively. See [Build Configurations](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#build-configurations) for discussion of this and other related topics.

For single-config generators, see [`CMAKE_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BUILD_TYPE.html#variable:CMAKE_BUILD_TYPE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
