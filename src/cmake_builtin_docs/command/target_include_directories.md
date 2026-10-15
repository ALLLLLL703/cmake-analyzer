CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html)

# target_include_directories

Add include directories to a target.

~~~
target_include_directories(<target> [SYSTEM] [AFTER|BEFORE]
  {INTERFACE|PUBLIC|PRIVATE} <dir...
  [{INTERFACE|PUBLIC|PRIVATE} <dir>...]...)
~~~

Specifies include directories to use when compiling a given target. The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

By using `AFTER` or `BEFORE` explicitly, you can select between appending and prepending, independent of the default.

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the following arguments. `PRIVATE` and `PUBLIC` items will populate the [`INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INCLUDE_DIRECTORIES.html#prop_tgt:INCLUDE_DIRECTORIES) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES) property of `<target>`. The following arguments specify include directories.

Added in version 3.11: Allow setting `INTERFACE` items on [IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets).

Repeated calls for the same `<target>` append items in the order called.

If `SYSTEM` is specified, the compiler will be told the directories are meant as system include directories on some platforms. This may have effects such as suppressing warnings or skipping the contained headers in dependency calculations (see compiler documentation). Additionally, system include directories are searched after normal include directories regardless of the order specified.

If `SYSTEM` is used together with `PUBLIC` or `INTERFACE`, the [`INTERFACE_SYSTEM_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_SYSTEM_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_SYSTEM_INCLUDE_DIRECTORIES) target property will be populated with the specified directories.

Arguments to `target_include_directories` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

Specified include directories may be absolute paths or relative paths. A relative path will be interpreted as relative to the current source directory (i.e. [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR)) and converted to an absolute path before storing it in the associated target property. If the path starts with a generator expression, it will always be assumed to be an absolute path (with one exception noted below) and will be used unmodified.

Include directories usage requirements commonly differ between the build-tree and the install-tree. The [`BUILD_INTERFACE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:BUILD_INTERFACE) and [`INSTALL_INTERFACE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:INSTALL_INTERFACE) generator expressions can be used to describe separate usage requirements based on the usage location. Relative paths are allowed within the [`INSTALL_INTERFACE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:INSTALL_INTERFACE) expression and are interpreted as relative to the installation prefix. Relative paths should not be used in [`BUILD_INTERFACE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:BUILD_INTERFACE) expressions because they will not be converted to absolute. For example:

~~~
target_include_directories(mylib PUBLIC
  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mylib>
  $<INSTALL_INTERFACE:include/mylib>  # <prefix>/include/mylib
)
~~~

## Creating Relocatable Packages

Note that it is not advisable to populate the [`INSTALL_INTERFACE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:INSTALL_INTERFACE) of the [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES) of a target with absolute paths to the include directories of dependencies. That would hard-code into installed packages the include directory paths for dependencies **as found on the machine the package was made on**.

The [`INSTALL_INTERFACE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:INSTALL_INTERFACE) of the [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES) is only suitable for specifying the required include directories for headers provided with the target itself, not those provided by the transitive dependencies listed in its [`INTERFACE_LINK_LIBRARIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_LINK_LIBRARIES.html#prop_tgt:INTERFACE_LINK_LIBRARIES) target property. Those dependencies should themselves be targets that specify their own header locations in [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES).

See the [Creating Relocatable Packages](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#creating-relocatable-packages) section of the [`cmake-packages(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-packages.7.html#manual:cmake-packages(7)) manual for discussion of additional care that must be taken when specifying usage requirements while creating packages for redistribution.

## See Also

- [`include_directories()`](https://cmake.org/cmake/help/v4.4/command/include_directories.html#command:include_directories)

- [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions)

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
