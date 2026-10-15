CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/install_targets.html)

# install_targets

Deprecated since version 3.0: Use the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) command instead.

This command has been superseded by the [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) command. It is provided for compatibility with older CMake code.

~~~
install_targets(<dir> [RUNTIME_DIRECTORY dir] target target)
~~~

Create rules to install the listed targets into the given directory. The directory `<dir>` is relative to the installation prefix, which is stored in the variable [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX). If `RUNTIME_DIRECTORY` is specified, then on systems with special runtime files (Windows DLL), the files will be copied to that directory.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
