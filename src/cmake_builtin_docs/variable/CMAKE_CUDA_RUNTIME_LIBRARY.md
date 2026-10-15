CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_RUNTIME_LIBRARY.html)

# CMAKE_CUDA_RUNTIME_LIBRARY

Added in version 3.17.

Select the CUDA runtime library for use when compiling and linking CUDA. This variable is used to initialize the [`CUDA_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_RUNTIME_LIBRARY.html#prop_tgt:CUDA_RUNTIME_LIBRARY) property on all targets as they are created.

The allowed case insensitive values are:

**`None`**

Link with `-cudart=none` or equivalent flag(s) to use no CUDA runtime library.

**`Shared`**

Link with `-cudart=shared` or equivalent flag(s) to use a dynamically-linked CUDA runtime library.

**`Static`**

Link with `-cudart=static` or equivalent flag(s) to use a statically-linked CUDA runtime library.

Contents of `CMAKE_CUDA_RUNTIME_LIBRARY` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

If this variable is not set then the [`CUDA_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_RUNTIME_LIBRARY.html#prop_tgt:CUDA_RUNTIME_LIBRARY) target property will not be set automatically. If that property is not set then CMake uses an appropriate default value based on the compiler to select the CUDA runtime library.

Note

This property has effect only when the `CUDA` language is enabled. To control the CUDA runtime linking when only using the CUDA SDK with the `C` or `C++` language we recommend using the [`FindCUDAToolkit`](https://cmake.org/cmake/help/v4.4/module/FindCUDAToolkit.html#module:FindCUDAToolkit) module.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
