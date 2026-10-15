CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_PLATFORM_TOOLSET_VERSION.html)

# CMAKE_VS_PLATFORM_TOOLSET_VERSION

Added in version 3.12.

Visual Studio Platform Toolset version.

The [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2017 and above allow to select minor versions of the same toolset. The toolset version number may be specified by a field in [`CMAKE_GENERATOR_TOOLSET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_TOOLSET.html#variable:CMAKE_GENERATOR_TOOLSET) of the form `version=14.11`. If none is specified CMake will choose a default toolset. The value may be empty if no minor version was selected and the default is used.

If the value is not empty, it is the version number that MSBuild uses in its `Microsoft.VCToolsVersion.*.props` file names.

Added in version 3.19.7: VS 16.9's toolset may also be specified as `14.28.16.9` because VS 16.10 uses the file name `Microsoft.VCToolsVersion.14.28.16.9.props`.

## Three-Component MSVC Toolset Versions

Added in version 3.19.7.

The `version=` field may be given a three-component toolset version such as `14.28.29910`, and CMake will convert it to the name used by MSBuild `Microsoft.VCToolsVersion.*.props` files. This is useful to distinguish between VS 16.8's `14.28.29333` toolset and VS 16.9's `14.28.29910` toolset. It also matches `vcvarsall`'s `-vcvars_ver=` behavior.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
