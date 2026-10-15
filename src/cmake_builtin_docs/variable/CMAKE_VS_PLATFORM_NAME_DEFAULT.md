CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_PLATFORM_NAME_DEFAULT.html)

# CMAKE_VS_PLATFORM_NAME_DEFAULT

Added in version 3.14.3.

Default for the Visual Studio target platform name for the current generator without considering the value of the [`CMAKE_GENERATOR_PLATFORM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_PLATFORM.html#variable:CMAKE_GENERATOR_PLATFORM) variable. For [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2017 and below this is always `Win32`. For VS 2019 and above this is based on the host platform.

See also the [`CMAKE_VS_PLATFORM_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_PLATFORM_NAME.html#variable:CMAKE_VS_PLATFORM_NAME) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
