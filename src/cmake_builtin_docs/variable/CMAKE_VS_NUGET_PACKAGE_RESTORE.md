CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_NUGET_PACKAGE_RESTORE.html)

# CMAKE_VS_NUGET_PACKAGE_RESTORE

Added in version 3.23.

When using [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators), this cache variable controls if msbuild should automatically attempt to restore NuGet packages prior to a build. NuGet packages can be defined using the [`VS_PACKAGE_REFERENCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_PACKAGE_REFERENCES.html#prop_tgt:VS_PACKAGE_REFERENCES) property on a target. If no package references are defined, this setting will do nothing.

The command line option [`--resolve-package-references`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-build-resolve-package-references) can be used alternatively to control the resolve behavior globally. This option will take precedence over the cache variable.

Targets that use the [`DOTNET_SDK`](https://cmake.org/cmake/help/v4.4/prop_tgt/DOTNET_SDK.html#prop_tgt:DOTNET_SDK) are required to run a restore before building. Disabling this option may cause the build to fail in such projects.

This setting is stored as a cache entry. Default value is `ON`.

See also the [`VS_PACKAGE_REFERENCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_PACKAGE_REFERENCES.html#prop_tgt:VS_PACKAGE_REFERENCES) property.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
