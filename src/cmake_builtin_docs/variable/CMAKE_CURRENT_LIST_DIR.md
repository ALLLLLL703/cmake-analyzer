CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_DIR.html)

# CMAKE_CURRENT_LIST_DIR

Full directory of the listfile currently being processed.

As CMake processes the listfiles in your project this variable will always be set to the directory where the listfile which is currently being processed ([`CMAKE_CURRENT_LIST_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_FILE.html#variable:CMAKE_CURRENT_LIST_FILE)) is located. The value has dynamic scope. When CMake starts processing commands in a source file it sets this variable to the directory where this file is located. When CMake finishes processing commands from the file it restores the previous value. Therefore the value of the variable inside a macro or function is the directory of the file invoking the bottom-most entry on the call stack, not the directory of the file containing the macro or function definition.

See also [`CMAKE_CURRENT_LIST_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_FILE.html#variable:CMAKE_CURRENT_LIST_FILE) and [`CMAKE_CURRENT_FUNCTION_LIST_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION_LIST_DIR.html#variable:CMAKE_CURRENT_FUNCTION_LIST_DIR).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
