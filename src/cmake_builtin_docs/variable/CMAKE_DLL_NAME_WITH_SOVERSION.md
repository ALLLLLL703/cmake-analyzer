CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DLL_NAME_WITH_SOVERSION.html)

# CMAKE_DLL_NAME_WITH_SOVERSION

Added in version 3.27.

This variable is used to initialize the [`DLL_NAME_WITH_SOVERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/DLL_NAME_WITH_SOVERSION.html#prop_tgt:DLL_NAME_WITH_SOVERSION) property on shared library targets for the Windows platform, which is selected when the [`WIN32`](https://cmake.org/cmake/help/v4.4/variable/WIN32.html#variable:WIN32) variable is set.

See this target property for additional information.

Please note that setting this variable has no effect if versioned filenames are globally disabled with the [`CMAKE_PLATFORM_NO_VERSIONED_SONAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PLATFORM_NO_VERSIONED_SONAME.html#variable:CMAKE_PLATFORM_NO_VERSIONED_SONAME) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
