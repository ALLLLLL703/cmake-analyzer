CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_RPATH_USE_LINK_PATH.html)

# CMAKE_INSTALL_RPATH_USE_LINK_PATH

Add paths to linker search and installed rpath.

`CMAKE_INSTALL_RPATH_USE_LINK_PATH` is a boolean that if set to `True` will append to the runtime search path (rpath) of installed binaries any directories outside the project that are in the linker search path or contain linked library files. The directories are appended after the value of the [`INSTALL_RPATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/INSTALL_RPATH.html#prop_tgt:INSTALL_RPATH) target property.

This variable is used to initialize the target property [`INSTALL_RPATH_USE_LINK_PATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/INSTALL_RPATH_USE_LINK_PATH.html#prop_tgt:INSTALL_RPATH_USE_LINK_PATH) for all targets.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
