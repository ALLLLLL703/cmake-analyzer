CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_compile_definitions.html)

# add_compile_definitions

Added in version 3.12.

Add preprocessor definitions to the compilation of source files.

~~~
add_compile_definitions(<definition> ...)
~~~

Adds preprocessor definitions to the compiler command line.

The preprocessor definitions are added to the [`COMPILE_DEFINITIONS`](https://cmake.org/cmake/help/v4.4/prop_dir/COMPILE_DEFINITIONS.html#prop_dir:COMPILE_DEFINITIONS) directory property for the current `CMakeLists` file. They are also added to the [`COMPILE_DEFINITIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_DEFINITIONS.html#prop_tgt:COMPILE_DEFINITIONS) target property for each target in the current `CMakeLists` file.

Definitions are specified using the syntax `VAR` or `VAR=value`. Function-style definitions are not supported. CMake will automatically escape the value correctly for the native build system (note that CMake language syntax may require escapes to specify some values).

Added in version 3.26: Any leading `-D` on an item will be removed.

Arguments to `add_compile_definitions` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

## See Also

- The command [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions) adds target-specific definitions.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
