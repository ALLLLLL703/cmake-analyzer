CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TASKING_TOOLSET.html)

# CMAKE_TASKING_TOOLSET

Added in version 3.25.

Select the Tasking toolset which provides the compiler

Architecture compilers are provided by different toolchains with incompatible versioning schemes. Set this variable in a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) so CMake can detect the compiler features correctly. If no toolset is specified, `Standalone` is assumed.

Due to the different versioning schemes, the compiler version ([`CMAKE_<LANG>_COMPILER_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_VERSION.html#variable:CMAKE_%3CLANG%3E_COMPILER_VERSION)) depends on the toolset and architecture in use. If projects can be built with multiple toolsets or architectures, the specified `CMAKE_TASKING_TOOLSET` and the automatically determined [`CMAKE_<LANG>_COMPILER_ARCHITECTURE_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ARCHITECTURE_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ARCHITECTURE_ID) must be taken into account when comparing against the [`CMAKE_<LANG>_COMPILER_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_VERSION.html#variable:CMAKE_%3CLANG%3E_COMPILER_VERSION).

**`TriCore`**

Compilers are provided by the TriCore toolset.

**`SmartCode`**

Compilers are provided by the SmartCode toolset.

**`Standalone`**

Compilers are provided by the standalone toolsets.

Note

For the TriCore architecture, the compiler from the TriCore toolset is selected as standalone compiler.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
