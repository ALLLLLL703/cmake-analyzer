CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_TYPE.html)

# CMAKE_\<LANG\>_USING_LINKER_\<TYPE\>

Added in version 3.29.

This variable defines how to specify the `<TYPE>` linker for the link step, as controlled by the [`CMAKE_LINKER_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINKER_TYPE.html#variable:CMAKE_LINKER_TYPE) variable or the [`LINKER_TYPE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINKER_TYPE.html#prop_tgt:LINKER_TYPE) target property. Depending on the value of the [`CMAKE_<LANG>_LINK_MODE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_MODE.html#variable:CMAKE_%3CLANG%3E_LINK_MODE) variable, `CMAKE_<LANG>_USING_LINKER_<TYPE>` can hold compiler flags for the link step, or the path to the linker tool.

Changed in version 4.0.

The type of information stored in this variable is now determined by the [`CMAKE_<LANG>_LINK_MODE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_MODE.html#variable:CMAKE_%3CLANG%3E_LINK_MODE) variable instead of the [`CMAKE_<LANG>_USING_LINKER_MODE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_USING_LINKER_MODE.html#variable:CMAKE_%3CLANG%3E_USING_LINKER_MODE) variable.

Note

The specified linker tool is generally expected to be accessible through the `PATH` environment variable.

For example, the `LLD` linker for `GNU` compilers is defined like so:

~~~
# CMAKE_C_LINK_MODE holds value "DRIVER"
set(CMAKE_C_USING_LINKER_LLD "-fuse-ld=lld")
~~~

On the `Windows` platform with `Clang` compilers simulating `MSVC` with `GNU` front-end:

~~~
# CMAKE_C_LINK_MODE holds value "DRIVER"
set(CMAKE_C_USING_LINKER_LLD "-fuse-ld=lld-link")
~~~

And for the `MSVC` compiler or `Clang` compilers simulating `MSVC` with `MSVC` front-end, the linker is invoked directly, not via the compiler front-end:

~~~
# CMAKE_C_LINK_MODE holds value "LINKER"
set(CMAKE_C_USING_LINKER_LLD "/path/to/lld-link.exe")
~~~

A custom linker type can also be defined, usually in a toolchain file:

~~~
set(CMAKE_LINKER_TYPE lld_launcher)
set(CMAKE_C_USING_LINKER_lld_launcher "-fuse-ld=/path/to/lld-launcher.sh")
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
