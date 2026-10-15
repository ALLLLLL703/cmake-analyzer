CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR.html)

# CMAKE_GENERATOR

The generator used to build the project. See [`cmake-generators(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#manual:cmake-generators(7)).

The name of the generator that is being used to generate the build files. (e.g. `Unix Makefiles`, `Ninja`, etc.)

The value of this variable should never be modified by project code. A generator may be selected via the [`cmake -G`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-G) option, interactively in [`cmake-gui(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake-gui.1.html#manual:cmake-gui(1)), or via the [`CMAKE_GENERATOR`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_GENERATOR.html#envvar:CMAKE_GENERATOR) environment variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
