CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_LINE.html)

# CMAKE_CURRENT_LIST_LINE

The line number of the current file being processed.

This is the line number of the file currently being processed by cmake.

If CMake is currently processing deferred calls scheduled by the [`cmake_language(DEFER)`](https://cmake.org/cmake/help/v4.4/command/cmake_language.html#defer) command, this variable evaluates to `DEFERRED` instead of a specific line number.

See also [`CMAKE_CURRENT_FUNCTION_LIST_LINE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION_LIST_LINE.html#variable:CMAKE_CURRENT_FUNCTION_LIST_LINE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
