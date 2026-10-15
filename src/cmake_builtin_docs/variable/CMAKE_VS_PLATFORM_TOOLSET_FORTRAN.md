CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_PLATFORM_TOOLSET_FORTRAN.html)

# CMAKE_VS_PLATFORM_TOOLSET_FORTRAN

Added in version 3.29.

Fortran compiler to be used by Visual Studio projects.

[Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) support selecting among Fortran compilers that have the required Visual Studio Integration feature installed. The compiler may be specified by a field in [`CMAKE_GENERATOR_TOOLSET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_TOOLSET.html#variable:CMAKE_GENERATOR_TOOLSET) of the form `fortran=...`. CMake provides the selected Fortran compiler in this variable.

If the field was not specified, the default depends on the generator:

- On [`Visual Studio 18 2026`](https://cmake.org/cmake/help/v4.4/generator/Visual%20Studio%2018%202026.html#generator:Visual%20Studio%2018%202026) and above, the default is `ifx`.

- On older [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators), the default is empty, which the Intel Visual Studio Integration interprets as equivalent to `ifort`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
