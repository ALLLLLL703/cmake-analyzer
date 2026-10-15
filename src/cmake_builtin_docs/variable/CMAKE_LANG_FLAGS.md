CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html)

# CMAKE_\<LANG\>_FLAGS

Language-wide flags for language `<LANG>` used when building for all configurations. These flags will be passed to all invocations of the compiler. This includes invocations that drive compiling and those that drive linking.

For each language, if this variable is not defined, it is initialized and stored in the cache using values from environment variables in combination with CMake's builtin defaults for the toolchain:

- `CMAKE_C_FLAGS`: Initialized by the [`CFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/CFLAGS.html#envvar:CFLAGS) environment variable.

- `CMAKE_CXX_FLAGS`: Initialized by the [`CXXFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/CXXFLAGS.html#envvar:CXXFLAGS) environment variable.

- `CMAKE_CUDA_FLAGS`: Initialized by the [`CUDAFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/CUDAFLAGS.html#envvar:CUDAFLAGS) environment variable.

- `CMAKE_Fortran_FLAGS`: Initialized by the [`FFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/FFLAGS.html#envvar:FFLAGS) environment variable.

- `CMAKE_CSharp_FLAGS`: Initialized by the [`CSFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/CSFLAGS.html#envvar:CSFLAGS) environment variable.

- `CMAKE_HIP_FLAGS`: Initialized by the [`HIPFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/HIPFLAGS.html#envvar:HIPFLAGS) environment variable.

- `CMAKE_ISPC_FLAGS`: Initialized by the [`ISPCFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/ISPCFLAGS.html#envvar:ISPCFLAGS) environment variable.

- `CMAKE_OBJC_FLAGS`: Initialized by the [`OBJCFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/OBJCFLAGS.html#envvar:OBJCFLAGS) environment variable.

- `CMAKE_OBJCXX_FLAGS`: Initialized by the [`OBJCXXFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/OBJCXXFLAGS.html#envvar:OBJCXXFLAGS) environment variable.

This value is a command-line string fragment. Therefore, multiple options should be separated by spaces, and options with spaces should be quoted.

The flags in this variable will be passed before those in the per-configuration [`CMAKE_<LANG>_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_FLAGS_%3CCONFIG%3E) variable. On invocations driving compiling, flags from both variables will be passed before flags added by commands such as [`add_compile_options()`](https://cmake.org/cmake/help/v4.4/command/add_compile_options.html#command:add_compile_options) and [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options). On invocations driving linking, they will be passed before flags added by commands such as [`add_link_options()`](https://cmake.org/cmake/help/v4.4/command/add_link_options.html#command:add_link_options) and [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
