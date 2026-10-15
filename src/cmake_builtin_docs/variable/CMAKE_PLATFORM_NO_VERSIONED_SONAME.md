CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PLATFORM_NO_VERSIONED_SONAME.html)

# CMAKE_PLATFORM_NO_VERSIONED_SONAME

Added in version 3.1.

This variable is used to globally control whether the [`VERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/VERSION.html#prop_tgt:VERSION) and [`SOVERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOVERSION.html#prop_tgt:SOVERSION) target properties should be used for shared libraries. When set to true, adding version information to each shared library target is disabled.

By default this variable is set only on platforms where CMake knows it is needed. On other platforms, the specified properties will be used for shared libraries.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
