CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html)

# CMAKE_\<LANG\>_LINKER_WRAPPER_FLAG

Added in version 3.13.

Defines the syntax of compiler driver option to pass options to the linker tool. It will be used to translate the `LINKER:` prefix in the link options (see [`add_link_options()`](https://cmake.org/cmake/help/v4.4/command/add_link_options.html#command:add_link_options) and [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)).

This variable holds a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of tokens. If a space (i.e. " ") is specified as last token, flag and `LINKER:` arguments will be specified as separate arguments to the compiler driver. The [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG_SEP) variable can be specified to manage concatenation of arguments.

For example, for `Clang` we have:

~~~
set (CMAKE_C_LINKER_WRAPPER_FLAG "-Xlinker" " ")
~~~

Specifying `"LINKER:-z,defs"` will be transformed in `-Xlinker -z -Xlinker defs`.

For `GNU GCC`:

~~~
set (CMAKE_C_LINKER_WRAPPER_FLAG "-Wl,")
set (CMAKE_C_LINKER_WRAPPER_FLAG_SEP ",")
~~~

Specifying `"LINKER:-z,defs"` will be transformed in `-Wl,-z,defs`.

And for `SunPro`:

~~~
set (CMAKE_C_LINKER_WRAPPER_FLAG "-Qoption" "ld" " ")
set (CMAKE_C_LINKER_WRAPPER_FLAG_SEP ",")
~~~

Specifying `"LINKER:-z,defs"` will be transformed in `-Qoption ld -z,defs`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
