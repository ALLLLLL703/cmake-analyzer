CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CPACK_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION.html)

# CPACK_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION

Ask CPack to error out as soon as a file with absolute `INSTALL DESTINATION` is encountered.

The fatal error is emitted before the installation of the offending file takes place. Some CPack generators, like `NSIS`, enforce this internally. This variable triggers the definition of [`CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION.html#variable:CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION) when CPack runs.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
