CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_LINKER.html)

# CMAKE_\<LANG\>_COMPILER_LINKER

Added in version 3.29.

The full path to the linker for `LANG`.

This is the command that will be used as the `<LANG>` linker.

This variable is not guaranteed to be defined for all linkers or languages.

Note

This variable is read-only. It must not be set by the user. To select a specific linker, use the [`CMAKE_LINKER_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINKER_TYPE.html#variable:CMAKE_LINKER_TYPE) variable or the [`LINKER_TYPE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINKER_TYPE.html#prop_tgt:LINKER_TYPE) target property.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
