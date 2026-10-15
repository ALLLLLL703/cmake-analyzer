CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_REDIRECTS_DIR.html)

# CMAKE_FIND_PACKAGE_REDIRECTS_DIR

Added in version 3.24.

This read-only variable specifies a directory that the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) command will check first before searching anywhere else for a module or config package file. A config package file in this directory will always be found in preference to any other Find module file or config package file.

The primary purpose of this variable is to facilitate integration between [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) and [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable). The latter command may create files in the `CMAKE_FIND_PACKAGE_REDIRECTS_DIR` directory when it populates a dependency. This allows subsequent calls to [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) for the same dependency to reuse the populated contents instead of trying to satisfy the dependency from somewhere external to the build. Projects may also want to write files into this directory in some situations (see [Integrating With find_package()](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#fetchcontent-find-package-integration-examples) for examples).

The directory that `CMAKE_FIND_PACKAGE_REDIRECTS_DIR` points to will always be erased and recreated empty at the start of every CMake run. Any files written into this directory during the CMake run will be lost the next time CMake configures the project.

`CMAKE_FIND_PACKAGE_REDIRECTS_DIR` is only set in CMake project mode. It is not set when CMake is run in script mode (i.e. [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P)).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
