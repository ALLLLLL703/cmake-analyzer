CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJDUMP.html)

# CMAKE_OBJDUMP

Path to the `objdump` executable on the host system. This tool, typically part of the Binutils collection on Unix-like systems, provides information about compiled object files.

This cache variable may be populated by CMake when project languages are enabled using the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) commands.

## See Also

- The [`file(GET_RUNTIME_DEPENDENCIES)`](https://cmake.org/cmake/help/v4.4/command/file.html#get-runtime-dependencies) command provides a more general way to get information from runtime binaries.

- The [`CPACK_OBJDUMP_EXECUTABLE`](https://cmake.org/cmake/help/v4.4/module/CPack.html#variable:CPACK_OBJDUMP_EXECUTABLE) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
