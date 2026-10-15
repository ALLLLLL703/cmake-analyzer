CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CODEBLOCKS_COMPILER_ID.html)

# CMAKE_CODEBLOCKS_COMPILER_ID

Added in version 3.11.

Change the compiler id in the generated CodeBlocks project files.

CodeBlocks uses its own compiler id string which differs from [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID). If this variable is left empty, CMake tries to recognize the CodeBlocks compiler id automatically. Otherwise the specified string is used in the CodeBlocks project file. See the CodeBlocks documentation for valid compiler id strings.

Other IDEs like QtCreator that also use the CodeBlocks generator may ignore this setting.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
