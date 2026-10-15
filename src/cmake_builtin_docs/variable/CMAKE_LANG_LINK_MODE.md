CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_MODE.html)

# CMAKE_\<LANG\>_LINK_MODE

Added in version 4.0.

Defines how the link step is done. The possible values are:

**`DRIVER`**

The compiler is used as driver for the link step.

**`LINKER`**

The linker is used directly for the link step.

This variable is read-only. Setting it is undefined behavior.

## See Also

- The [`CMAKE_<LANG>_USING_LINKER_<TYPE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_TYPE.html#variable:CMAKE_%3CLANG%3E_USING_LINKER_%3CTYPE%3E) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
