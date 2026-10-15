CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LINKER_FLAGS_INIT.html)

# CMAKE_SHARED_LINKER_FLAGS_INIT

Added in version 3.7.

Value used to initialize the [`CMAKE_SHARED_LINKER_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LINKER_FLAGS.html#variable:CMAKE_SHARED_LINKER_FLAGS) cache entry the first time a build tree is configured. This variable is meant to be set by a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE). CMake may prepend or append content to the value based on the environment and target platform.

See also the configuration-specific variable [`CMAKE_SHARED_LINKER_FLAGS_<CONFIG>_INIT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LINKER_FLAGS_CONFIG_INIT.html#variable:CMAKE_SHARED_LINKER_FLAGS_%3CCONFIG%3E_INIT).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
