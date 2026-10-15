CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DOTNET_TARGET_FRAMEWORK_VERSION.html)

# CMAKE_DOTNET_TARGET_FRAMEWORK_VERSION

Added in version 3.12.

Default value for [`DOTNET_TARGET_FRAMEWORK_VERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/DOTNET_TARGET_FRAMEWORK_VERSION.html#prop_tgt:DOTNET_TARGET_FRAMEWORK_VERSION) property of targets.

This variable is used to initialize the [`DOTNET_TARGET_FRAMEWORK_VERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/DOTNET_TARGET_FRAMEWORK_VERSION.html#prop_tgt:DOTNET_TARGET_FRAMEWORK_VERSION) property on all targets. See that target property for additional information. When set, [`CMAKE_DOTNET_TARGET_FRAMEWORK`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DOTNET_TARGET_FRAMEWORK.html#variable:CMAKE_DOTNET_TARGET_FRAMEWORK) takes precednece over this variable. See that variable or the associated target property [`DOTNET_TARGET_FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/DOTNET_TARGET_FRAMEWORK.html#prop_tgt:DOTNET_TARGET_FRAMEWORK) for additional information.

Setting `CMAKE_DOTNET_TARGET_FRAMEWORK_VERSION` may be necessary when working with `C#` and newer .NET framework versions to avoid referencing errors with the `ALL_BUILD` CMake target.

This variable is only evaluated for [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) VS 2010 and above.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
