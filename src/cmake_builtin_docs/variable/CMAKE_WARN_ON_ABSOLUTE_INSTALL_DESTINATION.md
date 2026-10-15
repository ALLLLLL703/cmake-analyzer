CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION.html)

# CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION

Ask `cmake_install.cmake` script to warn each time a file with absolute `INSTALL DESTINATION` is encountered.

This variable is used by CMake-generated `cmake_install.cmake` scripts. If one sets this variable to `ON` while running the script, it may get warning messages from the script.

Added in version 4.4: The [`CMD_INSTALL_ABSOLUTE_DESTINATION`](https://cmake.org/cmake/help/v4.4/diagnostic/CMD_INSTALL_ABSOLUTE_DESTINATION.html#diagnostic:CMD_INSTALL_ABSOLUTE_DESTINATION) diagnostic can be used to generate warnings for absolute install destinations at generate time.

See also [`CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION.html#variable:CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
