CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ARCHIVER_WRAPPER_FLAG.html)

# CMAKE_\<LANG\>_ARCHIVER_WRAPPER_FLAG

Added in version 4.0.

Defines the syntax of compiler driver option to pass options to the archiver tool. It will be used to translate the `ARCHIVER:` prefix in the static library options (see [`STATIC_LIBRARY_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/STATIC_LIBRARY_OPTIONS.html#prop_tgt:STATIC_LIBRARY_OPTIONS)).

This variable holds a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of tokens. If a space (i.e. " ") is specified as last token, flag and `ARCHIVER:` arguments will be specified as separate arguments to the compiler driver. The [`CMAKE_<LANG>_ARCHIVER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ARCHIVER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_ARCHIVER_WRAPPER_FLAG_SEP) variable can be specified to manage concatenation of arguments.

See [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) variable for examples of definitions because `CMAKE_<LANG>_ARCHIVER_WRAPPER_FLAG` use the same syntax.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
