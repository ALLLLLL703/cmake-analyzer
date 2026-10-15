CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_SIMULATE_VERSION.html)

# CMAKE_\<LANG\>_SIMULATE_VERSION

Version string of "simulated" compiler.

Some compilers simulate other compilers to serve as drop-in replacements. When CMake detects such a compiler it sets this variable to what would have been the [`CMAKE_<LANG>_COMPILER_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_VERSION.html#variable:CMAKE_%3CLANG%3E_COMPILER_VERSION) for the simulated compiler.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
