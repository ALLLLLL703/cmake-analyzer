CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_PLATFORM_TOOLSET_HOST_ARCHITECTURE.html)

# CMAKE_VS_PLATFORM_TOOLSET_HOST_ARCHITECTURE

Added in version 3.8.

Visual Studio preferred tool architecture.

The [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2013 and above support using either the 32-bit or 64-bit host toolchains by specifying a `host=x86` or `host=x64` value in the [`CMAKE_GENERATOR_TOOLSET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_TOOLSET.html#variable:CMAKE_GENERATOR_TOOLSET) option. CMake provides the selected toolchain architecture preference in this variable (`x86`, `x64`, `ARM64` or empty).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
