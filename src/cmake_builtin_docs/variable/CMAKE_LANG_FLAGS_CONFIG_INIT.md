CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG_INIT.html)

# CMAKE_\<LANG\>_FLAGS_\<CONFIG\>_INIT

Added in version 3.11.

Value used to initialize the [`CMAKE_<LANG>_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_FLAGS_%3CCONFIG%3E) cache entry the first time a build tree is configured for language `<LANG>`. This variable is meant to be set by a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE). CMake may prepend or append content to the value based on the environment and target platform.

See also [`CMAKE_<LANG>_FLAGS_INIT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_INIT.html#variable:CMAKE_%3CLANG%3E_FLAGS_INIT).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
