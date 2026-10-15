CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_NO_INSTALL_PREFIX.html)

# CMAKE_FIND_NO_INSTALL_PREFIX

Exclude the values of the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) and [`CMAKE_STAGING_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STAGING_PREFIX.html#variable:CMAKE_STAGING_PREFIX) variables from [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH). CMake adds these project-destination prefixes to [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH) by default in order to support building a series of dependent packages and installing them into a common prefix. Set `CMAKE_FIND_NO_INSTALL_PREFIX` to `TRUE` to suppress this behavior.

The [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH) is initialized on the first call to a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command. Therefore one must set `CMAKE_FIND_NO_INSTALL_PREFIX` before this in order to take effect. A user may set the variable as a cache entry on the command line to achieve this.

Note that the prefix(es) may still be searched for other reasons, such as being the same prefix as the CMake installation, or for being a built-in system prefix.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
