CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CPACK_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS.html)

# CPACK_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS

Added in version 3.11.

Default permissions for implicitly created directories during packaging.

This variable serves the same purpose during packaging as the [`CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS.html#variable:CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS) variable serves during installation (e.g. `make install`).

If `include(CPack)` is used then by default this variable is set to the content of [`CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS.html#variable:CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
