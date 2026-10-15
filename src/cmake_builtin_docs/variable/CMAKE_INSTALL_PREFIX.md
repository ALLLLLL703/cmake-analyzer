CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html)

# CMAKE_INSTALL_PREFIX

Install directory used by [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install).

If `make install` is invoked or `INSTALL` is built, this directory is prepended onto all install directories.

This variable defaults as follows:

- Added in version 3.29: If the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_INSTALL_PREFIX.html#envvar:CMAKE_INSTALL_PREFIX) environment variable is set, its value is used as default for this variable.

- `c:/Program Files/${PROJECT_NAME}` on Windows.

- `/usr/local` on UNIX platforms.

See [`CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT.html#variable:CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT) for how a project might choose its own default.

On UNIX one can use the `DESTDIR` mechanism in order to relocate the whole installation to a staging area. See the [`DESTDIR`](https://cmake.org/cmake/help/v4.4/envvar/DESTDIR.html#envvar:DESTDIR) environment variable for more information.

The installation prefix is also added to [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH) so that [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package), [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program), [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library), [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path), and [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file) will search the prefix for other software. This behavior can be disabled by setting the [`CMAKE_FIND_NO_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_NO_INSTALL_PREFIX.html#variable:CMAKE_FIND_NO_INSTALL_PREFIX) to `TRUE` before the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) invocation.

Note

Use the [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs) module to provide GNU-style options for the layout of directories within the installation.

The `CMAKE_INSTALL_PREFIX` may be defined when configuring a build tree to set its installation prefix. Or, when using the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command-line tool's [`--install`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-install) mode, one may specify a different prefix using the [`--prefix`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake--install-0) option:

~~~
cmake --install . --prefix /my/install/prefix
~~~

Note

When the project is using the [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs) module, there are some [special cases](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#gnuinstalldirs-special-cases) that are evaluated based on the value of the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) variable during the configuration phase. The results persist even if an alternative [`--prefix`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake--install-0) option is used during installation.

## See Also

- The [`--install-prefix`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-install-prefix) option.

- The [`$<INSTALL_PREFIX>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:INSTALL_PREFIX) generator expression.

- The [`installDir`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-configurepresets-installdir) field in CMake configure presets.

- The [`CPACK_PACKAGING_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CPACK_PACKAGING_INSTALL_PREFIX.html#variable:CPACK_PACKAGING_INSTALL_PREFIX) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
