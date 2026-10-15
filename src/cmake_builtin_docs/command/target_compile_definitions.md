CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html)

# target_compile_definitions

Add compile definitions to a target.

~~~
target_compile_definitions(<target>
  <INTERFACE|PUBLIC|PRIVATE> <definition>...
  [<INTERFACE|PUBLIC|PRIVATE> <definition>... ...])
~~~

Specifies compile definitions to use when compiling a given `<target>`. The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the following arguments. `PRIVATE` and `PUBLIC` items will populate the [`COMPILE_DEFINITIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_DEFINITIONS.html#prop_tgt:COMPILE_DEFINITIONS) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_COMPILE_DEFINITIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_COMPILE_DEFINITIONS.html#prop_tgt:INTERFACE_COMPILE_DEFINITIONS) property of `<target>`. The following arguments specify compile definitions. Repeated calls for the same `<target>` append items in the order called.

Added in version 3.11: Allow setting `INTERFACE` items on [IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets).

Arguments to `target_compile_definitions` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

Any leading `-D` on an item will be removed. Empty items are ignored. For example, the following are all equivalent:

~~~
target_compile_definitions(foo PUBLIC FOO)
target_compile_definitions(foo PUBLIC -DFOO)  # -D removed
target_compile_definitions(foo PUBLIC "" FOO) # "" ignored
target_compile_definitions(foo PUBLIC -D FOO) # -D becomes "", then ignored
~~~

Definitions may optionally have values:

~~~
target_compile_definitions(foo PUBLIC FOO=1)
~~~

Note that many compilers treat `-DFOO` as equivalent to `-DFOO=1`, but other tools may not recognize this in all circumstances (e.g. IntelliSense).

## See Also

- [`add_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/add_compile_definitions.html#command:add_compile_definitions)

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
