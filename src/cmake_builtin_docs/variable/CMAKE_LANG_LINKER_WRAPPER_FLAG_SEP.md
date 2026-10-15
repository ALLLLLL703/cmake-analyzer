CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html)

# CMAKE_\<LANG\>_LINKER_WRAPPER_FLAG_SEP

Added in version 3.13.

This variable is used with [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) variable to format `LINKER:` prefix in the link options (see [`add_link_options()`](https://cmake.org/cmake/help/v4.4/command/add_link_options.html#command:add_link_options) and [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)).

When specified, arguments of the `LINKER:` prefix will be concatenated using this value as separator.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
