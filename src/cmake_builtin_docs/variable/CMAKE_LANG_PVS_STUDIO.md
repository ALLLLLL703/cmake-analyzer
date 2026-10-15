CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_PVS_STUDIO.html)

# CMAKE_\<LANG\>_PVS_STUDIO

Added in version 4.3.

Default value for [`<LANG>_PVS_STUDIO`](https://cmake.org/cmake/help/v4.4/prop_tgt/LANG_PVS_STUDIO.html#prop_tgt:%3CLANG%3E_PVS_STUDIO) target property when `<LANG>` is `C` or `CXX`.

This variable is used to initialize the property on each target as it is created. For example:

~~~
set(CMAKE_CXX_PVS_STUDIO pvs-studio-analyzer analyze -a "GA\;OP")
add_executable(foo foo.cxx)
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
