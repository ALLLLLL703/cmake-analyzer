CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_HOST_COMPILER_ID.html)

# CMAKE_\<LANG\>_HOST_COMPILER_ID

Added in version 3.31.

This variable is available when `<LANG>` is `CUDA` or `HIP` and [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) is `NVIDIA`. It contains the identity of the host compiler invoked by `nvcc`, either by default or as specified by [`CMAKE_<LANG>_HOST_COMPILER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_HOST_COMPILER.html#variable:CMAKE_%3CLANG%3E_HOST_COMPILER), among possibilities documented by [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
