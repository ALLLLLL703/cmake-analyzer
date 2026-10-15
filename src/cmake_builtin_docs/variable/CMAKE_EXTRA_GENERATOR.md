CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXTRA_GENERATOR.html)

# CMAKE_EXTRA_GENERATOR

Deprecated since version 3.27: Support for [Extra Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#extra-generators) is deprecated and will be removed from a future version of CMake. IDEs may use the [`cmake-file-api(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-file-api.7.html#manual:cmake-file-api(7)) to view CMake-generated project build trees.

The extra generator used to build the project. See [`cmake-generators(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#manual:cmake-generators(7)).

When using the Eclipse, CodeBlocks, CodeLite, Kate or Sublime generators, CMake generates Makefiles ([`CMAKE_GENERATOR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR.html#variable:CMAKE_GENERATOR)) and additionally project files for the respective IDE. This IDE project file generator is stored in `CMAKE_EXTRA_GENERATOR` (e.g. `Eclipse CDT4`).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
