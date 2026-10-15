CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_link_options.html)

# target_link_options

Added in version 3.13.

Add options to the link step for an executable, shared library or module library target.

~~~
target_link_options(<target> [BEFORE]
  {INTERFACE|PUBLIC|PRIVATE} <item>...
  [{INTERFACE|PUBLIC|PRIVATE} <item>...]...)
~~~

The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

This command can be used to add any link options, but alternative commands exist to add libraries ([`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) or [`link_libraries()`](https://cmake.org/cmake/help/v4.4/command/link_libraries.html#command:link_libraries)). See documentation of the [`directory`](https://cmake.org/cmake/help/v4.4/prop_dir/LINK_OPTIONS.html#prop_dir:LINK_OPTIONS) and [`target`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_OPTIONS.html#prop_tgt:LINK_OPTIONS) `LINK_OPTIONS` properties.

Note

This command cannot be used to add options for static library targets, since they do not use a linker. To add archiver or MSVC librarian flags, see the [`STATIC_LIBRARY_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/STATIC_LIBRARY_OPTIONS.html#prop_tgt:STATIC_LIBRARY_OPTIONS) target property.

If `BEFORE` is specified, the content will be prepended to the property instead of being appended.

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the following arguments. `PRIVATE` and `PUBLIC` items will populate the [`LINK_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_OPTIONS.html#prop_tgt:LINK_OPTIONS) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_LINK_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_LINK_OPTIONS.html#prop_tgt:INTERFACE_LINK_OPTIONS) property of `<target>`. The following arguments specify link options. Repeated calls for the same `<target>` append items in the order called.

Note

[IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets) only support `INTERFACE` items.

Arguments to `target_link_options` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

## Host And Device Specific Link Options

Added in version 3.18: When a device link step is involved, which is controlled by [`CUDA_SEPARABLE_COMPILATION`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_SEPARABLE_COMPILATION.html#prop_tgt:CUDA_SEPARABLE_COMPILATION) and [`CUDA_RESOLVE_DEVICE_SYMBOLS`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_RESOLVE_DEVICE_SYMBOLS.html#prop_tgt:CUDA_RESOLVE_DEVICE_SYMBOLS) properties and policy [`CMP0105`](https://cmake.org/cmake/help/v4.4/policy/CMP0105.html#policy:CMP0105), the raw options will be delivered to the host and device link steps (wrapped in `-Xcompiler` or equivalent for device link). Options wrapped with [`$<DEVICE_LINK:...>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:DEVICE_LINK) generator expression will be used only for the device link step. Options wrapped with [`$<HOST_LINK:...>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:HOST_LINK) generator expression will be used only for the host link step.

## Option De-duplication

The final set of options used for a target is constructed by accumulating options from the current target and the usage requirements of its dependencies. The set of options is de-duplicated to avoid repetition.

Added in version 3.12: While beneficial for individual options, the de-duplication step can break up option groups. For example, `-option A -option B` becomes `-option A B`. One may specify a group of options using shell-like quoting along with a `SHELL:` prefix. The `SHELL:` prefix is dropped, and the rest of the option string is parsed using the [`separate_arguments()`](https://cmake.org/cmake/help/v4.4/command/separate_arguments.html#command:separate_arguments) `UNIX_COMMAND` mode. For example, `"SHELL:-option A" "SHELL:-option B"` becomes `-option A -option B`.

## Handling Compiler Driver Differences

To pass options to the linker tool, each compiler driver has its own syntax. The `LINKER:` prefix and `,` separator can be used to specify, in a portable way, options to pass to the linker tool. `LINKER:` is replaced by the appropriate driver option and `,` by the appropriate driver separator. The driver prefix and driver separator are given by the values of the [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) and [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG_SEP) variables.

For example, `"LINKER:-z,defs"` becomes `-Xlinker -z -Xlinker defs` for `Clang` and `-Wl,-z,defs` for `GNU GCC`.

The `LINKER:` prefix can be specified as part of a `SHELL:` prefix expression.

The `LINKER:` prefix supports, as an alternative syntax, specification of arguments using the `SHELL:` prefix and space as separator. The previous example then becomes `"LINKER:SHELL:-z defs"`.

Note

Specifying the `SHELL:` prefix anywhere other than at the beginning of the `LINKER:` prefix is not supported.

## See Also

- [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions)

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

- [`CMAKE_<LANG>_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html#variable:CMAKE_%3CLANG%3E_FLAGS) and [`CMAKE_<LANG>_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_FLAGS_%3CCONFIG%3E) add language-wide flags passed to all invocations of the compiler. This includes invocations that drive compiling and those that drive linking.

- Added in version 4.3: [`CMAKE_<LANG>_LINK_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS) and [`CMAKE_<LANG>_LINK_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS_%3CCONFIG%3E) add language-wide flags passed to all invocations of the compiler which drive linking.

- The [`CheckLinkerFlag`](https://cmake.org/cmake/help/v4.4/module/CheckLinkerFlag.html#module:CheckLinkerFlag) module to check whether a linker flag is supported by the compiler.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
