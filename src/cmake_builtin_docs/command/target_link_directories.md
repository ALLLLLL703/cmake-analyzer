CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html)

# target_link_directories

Added in version 3.13.

Add link directories to a target.

~~~
target_link_directories(<target> [BEFORE]
  {INTERFACE|PUBLIC|PRIVATE} <dir>...
  [{INTERFACE|PUBLIC|PRIVATE} <dir>...]...)
~~~

Specifies the paths in which the linker should search for libraries when linking a given target. Each item can be an absolute or relative path, with the latter being interpreted as relative to the current source directory. These items will be added to the link command.

The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the items that follow them. `PRIVATE` and `PUBLIC` items will populate the [`LINK_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_DIRECTORIES.html#prop_tgt:LINK_DIRECTORIES) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_LINK_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_LINK_DIRECTORIES.html#prop_tgt:INTERFACE_LINK_DIRECTORIES) property of `<target>` ([IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets) only support `INTERFACE` items). Each item specifies a link directory and will be converted to an absolute path if necessary before adding it to the relevant property. Repeated calls for the same `<target>` append items in the order called.

If `BEFORE` is specified, the content will be prepended to the relevant property instead of being appended.

Arguments to `target_link_directories` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

Note

This command is rarely necessary and should be avoided where there are other choices. Prefer to pass full absolute paths to libraries where possible, since this ensures the correct library will always be linked. The [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library) command provides the full path, which can generally be used directly in calls to [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries). Situations where a library search path may be needed include:

- Project generators like [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) where the user can switch target architecture at build time, but a full path to a library cannot be used because it only provides one architecture (i.e. it is not a universal binary).

- Libraries may themselves have other private library dependencies that expect to be found via `RPATH` mechanisms, but some linkers are not able to fully decode those paths (e.g. due to the presence of things like `$ORIGIN`).

## See Also

- [`link_directories()`](https://cmake.org/cmake/help/v4.4/command/link_directories.html#command:link_directories)

- [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions)

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
