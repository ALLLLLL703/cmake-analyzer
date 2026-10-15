CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_FILE.html)

# CMAKE_CURRENT_LIST_FILE

Full path to the listfile currently being processed.

As CMake processes the listfiles in your project this variable will always be set to the one currently being processed. The value has dynamic scope. When CMake starts processing commands in a source file it sets this variable to the location of the file. When CMake finishes processing commands from the file it restores the previous value. Therefore the value of the variable inside a macro or function is the file invoking the bottom-most entry on the call stack, not the file containing the macro or function definition.

See also [`CMAKE_PARENT_LIST_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PARENT_LIST_FILE.html#variable:CMAKE_PARENT_LIST_FILE) and [`CMAKE_CURRENT_FUNCTION_LIST_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION_LIST_FILE.html#variable:CMAKE_CURRENT_FUNCTION_LIST_FILE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
