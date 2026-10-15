CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_COLOR_DIAGNOSTICS.html)

# CMAKE_COLOR_DIAGNOSTICS

Added in version 3.24.

Enable color diagnostics throughout the generated build system.

This variable uses three states: `ON`, `OFF` and not defined.

When not defined:

- [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) initialize the [`CMAKE_COLOR_MAKEFILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_COLOR_MAKEFILE.html#variable:CMAKE_COLOR_MAKEFILE) variable to `ON`. It controls color buildsystem messages.

- GNU/Clang compilers are not invoked with any color diagnostics flag.

When `ON`:

- [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) produce color buildsystem messages by default. [`CMAKE_COLOR_MAKEFILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_COLOR_MAKEFILE.html#variable:CMAKE_COLOR_MAKEFILE) is not initialized, but may be explicitly set to `OFF` to disable color buildsystem messages.

- GNU/Clang compilers are invoked with a flag enabling color diagnostics (`-fcolor-diagnostics`).

When `OFF`:

- [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) do not produce color buildsystem messages by default. [`CMAKE_COLOR_MAKEFILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_COLOR_MAKEFILE.html#variable:CMAKE_COLOR_MAKEFILE) is not initialized, but may be explicitly set to `ON` to enable color buildsystem messages.

- GNU/Clang compilers are invoked with a flag disabling color diagnostics (`-fno-color-diagnostics`).

If the [`CMAKE_COLOR_DIAGNOSTICS`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_COLOR_DIAGNOSTICS.html#envvar:CMAKE_COLOR_DIAGNOSTICS) environment variable is set, its value is used. Otherwise, `CMAKE_COLOR_DIAGNOSTICS` is not defined by default.

See the [`CLICOLOR`](https://cmake.org/cmake/help/v4.4/envvar/CLICOLOR.html#envvar:CLICOLOR) and [`CLICOLOR_FORCE`](https://cmake.org/cmake/help/v4.4/envvar/CLICOLOR_FORCE.html#envvar:CLICOLOR_FORCE) environment variables to control color output from CMake command-line tools.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
