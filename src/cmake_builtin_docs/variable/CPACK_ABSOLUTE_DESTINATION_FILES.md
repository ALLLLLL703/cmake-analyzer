CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CPACK_ABSOLUTE_DESTINATION_FILES.html)

# CPACK_ABSOLUTE_DESTINATION_FILES

List of files which have been installed using an `ABSOLUTE DESTINATION` path.

This variable is a Read-Only variable which is set internally by CPack during installation and before packaging using [`CMAKE_ABSOLUTE_DESTINATION_FILES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ABSOLUTE_DESTINATION_FILES.html#variable:CMAKE_ABSOLUTE_DESTINATION_FILES) defined in `cmake_install.cmake` scripts. The value can be used within CPack project configuration file and/or `CPack<GEN>.cmake` file of `<GEN>` generator.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
