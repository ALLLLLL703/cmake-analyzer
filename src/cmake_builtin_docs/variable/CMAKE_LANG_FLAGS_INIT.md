CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_INIT.html)

# CMAKE_\<LANG\>_FLAGS_INIT

Added in version 3.7.

Value used to initialize the [`CMAKE_<LANG>_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html#variable:CMAKE_%3CLANG%3E_FLAGS) cache entry the first time a build tree is configured for language `<LANG>`. This variable is meant to be set by a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE). CMake may prepend or append content to the value based on the environment and target platform. For example, the contents of a `xxxFLAGS` environment variable will be prepended, where `xxx` will be language-specific but not necessarily the same as `<LANG>` (e.g. [`CXXFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/CXXFLAGS.html#envvar:CXXFLAGS) for `CXX`, [`FFLAGS`](https://cmake.org/cmake/help/v4.4/envvar/FFLAGS.html#envvar:FFLAGS) for `Fortran`, and so on). This value is a command-line string fragment. Therefore, multiple options should be separated by spaces, and options with spaces should be quoted.

See also the configuration-specific [`CMAKE_<LANG>_FLAGS_<CONFIG>_INIT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG_INIT.html#variable:CMAKE_%3CLANG%3E_FLAGS_%3CCONFIG%3E_INIT) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
