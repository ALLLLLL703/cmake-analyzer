CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_TARGET_FRAMEWORK_TARGETS_VERSION.html)

# CMAKE_VS_TARGET_FRAMEWORK_TARGETS_VERSION

Added in version 3.22.

Visual Studio target framework targets version.

In some cases, the [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) may use an explicit value for the MSBuild `TargetFrameworkTargetsVersion` setting in `.csproj` files. CMake provides the chosen value in this variable.

See also [`CMAKE_VS_TARGET_FRAMEWORK_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_TARGET_FRAMEWORK_VERSION.html#variable:CMAKE_VS_TARGET_FRAMEWORK_VERSION) and [`CMAKE_VS_TARGET_FRAMEWORK_IDENTIFIER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_TARGET_FRAMEWORK_IDENTIFIER.html#variable:CMAKE_VS_TARGET_FRAMEWORK_IDENTIFIER).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
