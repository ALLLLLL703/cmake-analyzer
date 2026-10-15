CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_FRAMEWORK.html)

# CMAKE_FIND_FRAMEWORK

Added in version 3.4.

This variable affects how `find_*` commands choose between macOS Frameworks and unix-style package components.

On Darwin or systems supporting macOS Frameworks, the `CMAKE_FIND_FRAMEWORK` variable can be set to empty or one of the following:

**`FIRST`**

Try to find frameworks before standard libraries or headers. This is the default on Darwin.

**`LAST`**

Try to find frameworks after standard libraries or headers.

**`ONLY`**

Only try to find frameworks.

**`NEVER`**

Never try to find frameworks.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
