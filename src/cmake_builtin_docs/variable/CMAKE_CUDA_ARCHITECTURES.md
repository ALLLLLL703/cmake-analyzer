CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_ARCHITECTURES.html)

# CMAKE_CUDA_ARCHITECTURES

Added in version 3.18.

Default value for [`CUDA_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_ARCHITECTURES.html#prop_tgt:CUDA_ARCHITECTURES) property of targets.

Initialized by the [`CUDAARCHS`](https://cmake.org/cmake/help/v4.4/envvar/CUDAARCHS.html#envvar:CUDAARCHS) environment variable if set. Otherwise as follows depending on [`CMAKE_CUDA_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID):

- For `Clang`: the oldest architecture that works.

- For `NVIDIA`: the default architecture chosen by the compiler. See policy [`CMP0104`](https://cmake.org/cmake/help/v4.4/policy/CMP0104.html#policy:CMP0104).

Users are encouraged to override this, as the default varies across compilers and compiler versions.

This variable is used to initialize the [`CUDA_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_ARCHITECTURES.html#prop_tgt:CUDA_ARCHITECTURES) property on all targets. See the target property for additional information.

## Examples

~~~
cmake_minimum_required(VERSION)

if(NOT DEFINED CMAKE_CUDA_ARCHITECTURES)
  set(CMAKE_CUDA_ARCHITECTURES 75)
endif()

project(example LANGUAGES CUDA)
~~~

`CMAKE_CUDA_ARCHITECTURES` will default to `75` unless overridden by the user.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
