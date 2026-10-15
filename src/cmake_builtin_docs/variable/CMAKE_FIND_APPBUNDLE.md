CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_APPBUNDLE.html)

# CMAKE_FIND_APPBUNDLE

Added in version 3.4.

This variable affects how `find_*` commands choose between macOS Application Bundles and unix-style package components.

On Darwin or systems supporting macOS Application Bundles, the `CMAKE_FIND_APPBUNDLE` variable can be set to empty or one of the following:

**`FIRST`**

Try to find application bundles before standard programs. This is the default on Darwin.

**`LAST`**

Try to find application bundles after standard programs.

**`ONLY`**

Only try to find application bundles.

**`NEVER`**

Never try to find application bundles.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
