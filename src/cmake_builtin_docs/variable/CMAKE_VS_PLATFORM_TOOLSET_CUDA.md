CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_PLATFORM_TOOLSET_CUDA.html)

# CMAKE_VS_PLATFORM_TOOLSET_CUDA

Added in version 3.9.

NVIDIA CUDA Toolkit version whose Visual Studio toolset to use.

The [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2010 and above support using a CUDA toolset provided by a CUDA Toolkit. The toolset version number may be specified by a field in [`CMAKE_GENERATOR_TOOLSET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_TOOLSET.html#variable:CMAKE_GENERATOR_TOOLSET) of the form `cuda=8.0`. Or it is automatically detected if a path to a standalone CUDA directory is specified in the form `cuda=C:\path\to\cuda`. If none is specified CMake will choose a default version. CMake provides the selected CUDA toolset version in this variable. The value may be empty if no CUDA Toolkit with Visual Studio integration is installed.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
