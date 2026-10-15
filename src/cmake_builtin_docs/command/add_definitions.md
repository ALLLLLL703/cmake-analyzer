CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_definitions.html)

# add_definitions

Add `-D` define flags to the compilation of source files.

~~~
add_definitions(-DFOO -DBAR ...)
~~~

Adds definitions to the compiler command line for targets in the current directory, whether added before or after this command is invoked, and for the ones in sub-directories added after. This command can be used to add any flags, but it is intended to add preprocessor definitions.

Note

This command has been superseded by alternatives:

- Use [`add_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/add_compile_definitions.html#command:add_compile_definitions) to add preprocessor definitions.

- Use [`include_directories()`](https://cmake.org/cmake/help/v4.4/command/include_directories.html#command:include_directories) to add include directories.

- Use [`add_compile_options()`](https://cmake.org/cmake/help/v4.4/command/add_compile_options.html#command:add_compile_options) to add other options.

Flags beginning in `-D` or `/D` that look like preprocessor definitions are automatically added to the [`COMPILE_DEFINITIONS`](https://cmake.org/cmake/help/v4.4/prop_dir/COMPILE_DEFINITIONS.html#prop_dir:COMPILE_DEFINITIONS) directory property for the current directory. Definitions with non-trivial values may be left in the set of flags instead of being converted for reasons of backwards compatibility. See documentation of the [`directory`](https://cmake.org/cmake/help/v4.4/prop_dir/COMPILE_DEFINITIONS.html#prop_dir:COMPILE_DEFINITIONS), [`target`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_DEFINITIONS.html#prop_tgt:COMPILE_DEFINITIONS), [`source file`](https://cmake.org/cmake/help/v4.4/prop_sf/COMPILE_DEFINITIONS.html#prop_sf:COMPILE_DEFINITIONS) `COMPILE_DEFINITIONS` properties for details on adding preprocessor definitions to specific scopes and configurations.

## See Also

- The [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
