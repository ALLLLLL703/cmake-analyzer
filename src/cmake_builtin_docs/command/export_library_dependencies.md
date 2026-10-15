CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/export_library_dependencies.html)

# export_library_dependencies

Disallowed since version 3.0. See CMake Policy [`CMP0033`](https://cmake.org/cmake/help/v4.4/policy/CMP0033.html#policy:CMP0033).

Use [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) or [`export()`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) command.

This command generates an old-style library dependencies file. Projects requiring CMake 2.6 or later should not use the command. Use instead the [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) command to help export targets from an installation tree and the [`export()`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) command to export targets from a build tree.

The old-style library dependencies file does not take into account per-configuration names of libraries or the [`LINK_INTERFACE_LIBRARIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_INTERFACE_LIBRARIES.html#prop_tgt:LINK_INTERFACE_LIBRARIES) target property.

~~~
export_library_dependencies(<file> [APPEND])
~~~

Create a file named `<file>` that can be included into a CMake listfile with the INCLUDE command. The file will contain a number of SET commands that will set all the variables needed for library dependency information. This should be the last command in the top level CMakeLists.txt file of the project. If the `APPEND` option is specified, the SET commands will be appended to the given file instead of replacing it.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
