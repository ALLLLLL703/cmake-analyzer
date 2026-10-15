CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_LAUNCHER.html)

# CMAKE_\<LANG\>_COMPILER_LAUNCHER

Added in version 3.4.

Default value for [`<LANG>_COMPILER_LAUNCHER`](https://cmake.org/cmake/help/v4.4/prop_tgt/LANG_COMPILER_LAUNCHER.html#prop_tgt:%3CLANG%3E_COMPILER_LAUNCHER) target property. This variable is used to initialize the property on each target as it is created. This is done only when `<LANG>` is `C`, `CXX`, `Fortran`, `HIP`, `ISPC`, `OBJC`, `OBJCXX`, or `CUDA`.

This variable is initialized to the [`CMAKE_<LANG>_COMPILER_LAUNCHER`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_LANG_COMPILER_LAUNCHER.html#envvar:CMAKE_%3CLANG%3E_COMPILER_LAUNCHER) environment variable if it is set.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
