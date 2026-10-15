CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html)

# target_compile_options

Add compile options to a target.

~~~
target_compile_options(<target> [BEFORE]
  {INTERFACE|PUBLIC|PRIVATE} <item>...
  [{INTERFACE|PUBLIC|PRIVATE} <item>...]...)
~~~

Adds options to the [`COMPILE_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_OPTIONS.html#prop_tgt:COMPILE_OPTIONS) or [`INTERFACE_COMPILE_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_COMPILE_OPTIONS.html#prop_tgt:INTERFACE_COMPILE_OPTIONS) target properties. These options are used when compiling the given `<target>`, which must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

Note

These options are not used when linking the target. See the [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options) command for that.

## Arguments

If `BEFORE` is specified, the content will be prepended to the property instead of being appended. See policy [`CMP0101`](https://cmake.org/cmake/help/v4.4/policy/CMP0101.html#policy:CMP0101) which affects whether `BEFORE` will be ignored in certain cases.

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the following arguments. `PRIVATE` and `PUBLIC` items will populate the [`COMPILE_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_OPTIONS.html#prop_tgt:COMPILE_OPTIONS) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_COMPILE_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_COMPILE_OPTIONS.html#prop_tgt:INTERFACE_COMPILE_OPTIONS) property of `<target>`. The following arguments specify compile options. Repeated calls for the same `<target>` append items in the order called.

Added in version 3.11: Allow setting `INTERFACE` items on [IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets).

Arguments to `target_compile_options` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

## Option De-duplication

The final set of options used for a target is constructed by accumulating options from the current target and the usage requirements of its dependencies. The set of options is de-duplicated to avoid repetition.

Added in version 3.12: While beneficial for individual options, the de-duplication step can break up option groups. For example, `-option A -option B` becomes `-option A B`. One may specify a group of options using shell-like quoting along with a `SHELL:` prefix. The `SHELL:` prefix is dropped, and the rest of the option string is parsed using the [`separate_arguments()`](https://cmake.org/cmake/help/v4.4/command/separate_arguments.html#command:separate_arguments) `UNIX_COMMAND` mode. For example, `"SHELL:-option A" "SHELL:-option B"` becomes `-option A -option B`.

## See Also

- This command can be used to add any options. However, for adding preprocessor definitions and include directories it is recommended to use the more specific commands [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions) and [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories).

- For directory-wide settings, there is the command [`add_compile_options()`](https://cmake.org/cmake/help/v4.4/command/add_compile_options.html#command:add_compile_options).

- For file-specific settings, there is the source file property [`COMPILE_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_sf/COMPILE_OPTIONS.html#prop_sf:COMPILE_OPTIONS).

- This command adds compile options for all languages in a target. Use the [`COMPILE_LANGUAGE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:COMPILE_LANGUAGE) generator expression to specify per-language compile options.

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

- [`CMAKE_<LANG>_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html#variable:CMAKE_%3CLANG%3E_FLAGS) and [`CMAKE_<LANG>_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_FLAGS_%3CCONFIG%3E) add language-wide flags passed to all invocations of the compiler. This includes invocations that drive compiling and those that drive linking.

- The [`CheckCompilerFlag`](https://cmake.org/cmake/help/v4.4/module/CheckCompilerFlag.html#module:CheckCompilerFlag) module to check whether the compiler supports a given flag.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
