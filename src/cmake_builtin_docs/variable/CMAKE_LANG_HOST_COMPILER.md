CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_HOST_COMPILER.html)

# CMAKE_\<LANG\>_HOST_COMPILER

Added in version 3.10: `CMAKE_CUDA_HOST_COMPILER`

Added in version 3.28: `CMAKE_HIP_HOST_COMPILER`

This variable is available when `<LANG>` is `CUDA` or `HIP`.

When [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) is `NVIDIA`, `CMAKE_<LANG>_HOST_COMPILER` selects the compiler executable to use when compiling host code for `CUDA` or `HIP` language files. This maps to the `nvcc -ccbin` option.

The `CMAKE_<LANG>_HOST_COMPILER` variable may be set explicitly before CUDA or HIP is first enabled by a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command. This can be done via `-DCMAKE_<LANG>_HOST_COMPILER=...` on the command line or in a [toolchain file](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-toolchain). Or, one may set the [`CUDAHOSTCXX`](https://cmake.org/cmake/help/v4.4/envvar/CUDAHOSTCXX.html#envvar:CUDAHOSTCXX) or [`HIPHOSTCXX`](https://cmake.org/cmake/help/v4.4/envvar/HIPHOSTCXX.html#envvar:HIPHOSTCXX) environment variable to provide a default value.

Once the CUDA or HIP language is enabled, the `CMAKE_<LANG>_HOST_COMPILER` variable is read-only and changes to it are undefined behavior.

Note

Since `CMAKE_<LANG>_HOST_COMPILER` is meaningful only when the [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) is `NVIDIA`, it does not make sense to set `CMAKE_<LANG>_HOST_COMPILER` without also setting `CMAKE_<LANG>_COMPILER` to NVCC.

Note

Projects should not try to set `CMAKE_<LANG>_HOST_COMPILER` to match [`CMAKE_CXX_COMPILER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER.html#variable:CMAKE_%3CLANG%3E_COMPILER) themselves. It is the end-user's responsibility, not the project's, to ensure that NVCC targets the same ABI as the C++ compiler.

Note

Ignored when using [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators).

See the [`CMAKE_<LANG>_HOST_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_HOST_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_HOST_COMPILER_ID) and [`CMAKE_<LANG>_HOST_COMPILER_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_HOST_COMPILER_VERSION.html#variable:CMAKE_%3CLANG%3E_HOST_COMPILER_VERSION) variables for information about the host compiler used by `nvcc`, whether by default or specified by `CMAKE_<LANG>_HOST_COMPILER`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
