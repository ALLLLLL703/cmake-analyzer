CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BUILD_TYPE.html)

# CMAKE_BUILD_TYPE

Specifies the build type on single-configuration generators (e.g. [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) or [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja)). Typical values include `Debug`, `Release`, `RelWithDebInfo` and `MinSizeRel`, but custom build types can also be defined.

This variable is initialized by the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command called in a project when a new build tree is first created. If the [`CMAKE_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_BUILD_TYPE.html#envvar:CMAKE_BUILD_TYPE) environment variable is set, its value is used. Otherwise, a toolchain-specific default is chosen when a language is enabled. The default value is often an empty string, but this is usually not desirable and one of the other standard build types is usually more appropriate.

Depending on the situation, the value of this variable may be treated case-sensitively or case-insensitively. See [Build Configurations](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#build-configurations) for discussion of this and other related topics.

For multi-config generators, see [`CMAKE_CONFIGURATION_TYPES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CONFIGURATION_TYPES.html#variable:CMAKE_CONFIGURATION_TYPES).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
