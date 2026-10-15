CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_SIMULATE_ID.html)

# CMAKE_\<LANG\>_SIMULATE_ID

Identification string of the "simulated" compiler.

Some compilers simulate other compilers to serve as drop-in replacements. When CMake detects such a compiler it sets this variable to what would have been the [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) for the simulated compiler.

Note

In other words, this variable describes the ABI compatibility of the generated code.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
