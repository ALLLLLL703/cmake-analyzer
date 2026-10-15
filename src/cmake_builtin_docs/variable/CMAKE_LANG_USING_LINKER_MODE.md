CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_MODE.html)

# CMAKE_\<LANG\>_USING_LINKER_MODE

Deprecated since version 4.0: This variable is no longer used. The type of information stored in the [`CMAKE_<LANG>_USING_LINKER_<TYPE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_TYPE.html#variable:CMAKE_%3CLANG%3E_USING_LINKER_%3CTYPE%3E) variable is determined by the [`CMAKE_<LANG>_LINK_MODE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_MODE.html#variable:CMAKE_%3CLANG%3E_LINK_MODE) variable.

Added in version 3.29.

This variable controls how the value of the [`CMAKE_<LANG>_USING_LINKER_<TYPE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_TYPE.html#variable:CMAKE_%3CLANG%3E_USING_LINKER_%3CTYPE%3E) variable should be interpreted. The supported linker mode values are:

**`FLAG`**

[`CMAKE_<LANG>_USING_LINKER_<TYPE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_TYPE.html#variable:CMAKE_%3CLANG%3E_USING_LINKER_%3CTYPE%3E) holds a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of flags to be passed to the compiler frontend. This is also the default behavior if `CMAKE_<LANG>_USING_LINKER_MODE` is not set.

**`TOOL`**

[`CMAKE_<LANG>_USING_LINKER_<TYPE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_TYPE.html#variable:CMAKE_%3CLANG%3E_USING_LINKER_%3CTYPE%3E) holds the path to the linker tool.

Warning

The variable must be set accordingly to how CMake manage the link step:

- value `TOOL` is expected and required when the linker is used directly for the link step.

- value `FLAG` is expected or the variable not set when the compiler is used as driver for the link step.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
