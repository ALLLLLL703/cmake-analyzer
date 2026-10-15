CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HIP_PLATFORM.html)

# CMAKE_HIP_PLATFORM

Added in version 3.28.

GPU platform for which HIP language sources are to be compiled.

The value must be one of:

**`amd`**

AMD GPUs

**`nvidia`**

NVIDIA GPUs

**`spirv`**

Added in version 4.3.

SPIR-V target (e.g., chipStar)

If not specified, a default is computed via `hipconfig --platform`.

[`CMAKE_HIP_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HIP_ARCHITECTURES.html#variable:CMAKE_HIP_ARCHITECTURES) entries are interpreted with as architectures of the GPU platform.

[`CMAKE_HIP_COMPILER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER.html#variable:CMAKE_%3CLANG%3E_COMPILER) must target the same GPU platform.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
