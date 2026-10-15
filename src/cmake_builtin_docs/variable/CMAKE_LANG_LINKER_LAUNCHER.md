CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_LAUNCHER.html)

# CMAKE_\<LANG\>_LINKER_LAUNCHER

Added in version 3.21.

Default value for [`<LANG>_LINKER_LAUNCHER`](https://cmake.org/cmake/help/v4.4/prop_tgt/LANG_LINKER_LAUNCHER.html#prop_tgt:%3CLANG%3E_LINKER_LAUNCHER) target property. This variable is used to initialize the property on each target as it is created. This is done only when `<LANG>` is one of:

- `C`

- `CXX`

- `OBJC`

- `OBJCXX`

- `Fortran`

    

  Added in version 4.1.

This variable is initialized to the [`CMAKE_<LANG>_LINKER_LAUNCHER`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_LANG_LINKER_LAUNCHER.html#envvar:CMAKE_%3CLANG%3E_LINKER_LAUNCHER) environment variable if it is set.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
