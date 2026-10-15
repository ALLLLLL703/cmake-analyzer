CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_sources.html)

# target_sources

Added in version 3.1.

Add sources to a target.

~~~
target_sources(<target>
  {INTERFACE|PUBLIC|PRIVATE} <source>...
  [{INTERFACE|PUBLIC|PRIVATE} <source>...]...)
~~~

Specifies sources to use when building a target and/or its dependents. The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) or [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets). The `<source>` items may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

Added in version 3.20: `<target>` can be a custom target.

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the source file paths (`<source>...`) that follow them. `PRIVATE` and `PUBLIC` sources will populate the [`SOURCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCES.html#prop_tgt:SOURCES) property of `<target>`, which are used when building the target itself. `PUBLIC` and `INTERFACE` sources will populate the [`INTERFACE_SOURCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_SOURCES.html#prop_tgt:INTERFACE_SOURCES) property of `<target>`, which are used when building dependents. A target created by [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) can only have `PRIVATE` scope.

Repeated calls for the same `<target>` append sources in the order called.

Added in version 3.3: Allow exporting targets with [`INTERFACE_SOURCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_SOURCES.html#prop_tgt:INTERFACE_SOURCES).

Added in version 3.11: Allow setting `INTERFACE` sources on [IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets).

Changed in version 3.13: Relative source file paths are interpreted as being relative to the current source directory (i.e. [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR)). See policy [`CMP0076`](https://cmake.org/cmake/help/v4.4/policy/CMP0076.html#policy:CMP0076).

A path that begins with a generator expression is left unmodified. When a target's [`SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCE_DIR.html#prop_tgt:SOURCE_DIR) property differs from [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR), use absolute paths in generator expressions to ensure the sources are correctly assigned to the target.

~~~
# WRONG: starts with generator expression, but relative path used
target_sources(MyTarget PRIVATE "$<$<CONFIG:Debug>:dbgsrc.cpp>")

# CORRECT: absolute path used inside the generator expression
target_sources(MyTarget PRIVATE "$<$<CONFIG:Debug>:${CMAKE_CURRENT_SOURCE_DIR}/dbgsrc.cpp>")
~~~

See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

## File Sets

Added in version 3.23.

~~~
target_sources(<target>
  [{INTERFACE|PUBLIC|PRIVATE}
   [FILE_SET <set> [TYPE <type>] [BASE_DIRS <dirs>...] [FILES <files>...]]...
  ]...)
~~~

Adds a file set to a target, or adds files to an existing file set. Targets have zero or more named file sets. Each file set has a name, a type, a scope of `INTERFACE`, `PUBLIC`, or `PRIVATE`, one or more base directories, and files within those directories.

Changed in version 4.4: A file may belong to at most one non-`HEADERS` file set in a target. See policy [`CMP0211`](https://cmake.org/cmake/help/v4.4/policy/CMP0211.html#policy:CMP0211).

The acceptable types include:

`HEADERS`

Sources intended to be used via a language's `#include` mechanism.

**`SOURCES`**

Added in version 4.4.

Specifies sources to use when building a target and/or its dependents. With the scope `PRIVATE` and `PUBLIC`, `<files>` will populate the [`SOURCES`](https://cmake.org/cmake/help/v4.4/prop_fs/SOURCES.html#prop_fs:SOURCES) property of `<set>`, which are used when building the target itself. With the scope `PUBLIC` and `INTERFACE`, `<files>` will populate the [`INTERFACE_SOURCES`](https://cmake.org/cmake/help/v4.4/prop_fs/INTERFACE_SOURCES.html#prop_fs:INTERFACE_SOURCES) property of `<set>`, which are used when building dependents. The sources specified by the [`INTERFACE_SOURCES`](https://cmake.org/cmake/help/v4.4/prop_fs/INTERFACE_SOURCES.html#prop_fs:INTERFACE_SOURCES) property are propagated, transitively, to all the dependents.

**`CXX_MODULES`**

Added in version 3.28.

Sources which contain C++ interface module or partition units (i.e., those using the `export` keyword). This file set type may not have an `INTERFACE` scope except on `IMPORTED` targets.

The optional default file sets are named after their type. The target may not be a custom target or, for `HEADERS` and `CXX_MODULES` types, a [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) target.

Files in a `PRIVATE` or `PUBLIC` file set are marked as source files for the purposes of IDE integration. Additionally, files in `HEADERS` file sets have their [`HEADER_FILE_ONLY`](https://cmake.org/cmake/help/v4.4/prop_sf/HEADER_FILE_ONLY.html#prop_sf:HEADER_FILE_ONLY) property set to `TRUE`. Files in an `INTERFACE` or `PUBLIC` file set can be installed with the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) command, and exported with the [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) and [`export()`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) commands.

Each `target_sources(FILE_SET)` entry starts with `INTERFACE`, `PUBLIC`, or `PRIVATE` and accepts the following arguments:

`FILE_SET <set>`

The name of the file set to create or add to. It must contain only letters, numbers and underscores. Names starting with a capital letter are reserved for built-in file sets predefined by CMake. The only predefined set names are those matching the acceptable types. All other set names must not start with a capital letter or underscore.

`TYPE <type>`

Every file set is associated with a particular type of file. Only types specified above may be used and it is an error to specify anything else. As a special case, if the name of the file set is one of the types, the type does not need to be specified and the `TYPE <type>` arguments can be omitted. For all other file set names, `TYPE` is required.

`BASE_DIRS <dirs>...`

An optional list of base directories of the file set. Any relative path is treated as relative to the current source directory (i.e. [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR)). If no `BASE_DIRS` are specified when the file set is first created, the value of [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR) is added. This argument supports [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

No two base directories for a file set may be sub-directories of each other. This requirement must be met across all base directories added to a file set, not just those within a single call to `target_sources()`.

`FILES <files>...`

An optional list of files to add to the file set. Each file must be in one of the base directories, or a subdirectory of one of the base directories. This argument supports [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

If relative paths are specified, they are considered relative to [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR) at the time `target_sources()` is called. An exception to this is a path starting with `$<`. Such paths are treated as relative to the target's source directory after evaluation of generator expressions.

The following target properties are set by `target_sources(FILE_SET)`, but they should not generally be manipulated directly:

For file sets of type `HEADERS`:

- [`HEADER_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/HEADER_SETS.html#prop_tgt:HEADER_SETS)

- [`INTERFACE_HEADER_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_HEADER_SETS.html#prop_tgt:INTERFACE_HEADER_SETS)

- [`HEADER_SET`](https://cmake.org/cmake/help/v4.4/prop_tgt/HEADER_SET.html#prop_tgt:HEADER_SET)

- [`HEADER_SET_<NAME>`](https://cmake.org/cmake/help/v4.4/prop_tgt/HEADER_SET_NAME.html#prop_tgt:HEADER_SET_%3CNAME%3E)

- [`HEADER_DIRS`](https://cmake.org/cmake/help/v4.4/prop_tgt/HEADER_DIRS.html#prop_tgt:HEADER_DIRS)

- [`HEADER_DIRS_<NAME>`](https://cmake.org/cmake/help/v4.4/prop_tgt/HEADER_DIRS_NAME.html#prop_tgt:HEADER_DIRS_%3CNAME%3E)

For file sets of type `SOURCES`:

- [`SOURCE_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCE_SETS.html#prop_tgt:SOURCE_SETS)

- [`INTERFACE_SOURCE_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_SOURCE_SETS.html#prop_tgt:INTERFACE_SOURCE_SETS)

- [`SOURCE_SET`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCE_SET.html#prop_tgt:SOURCE_SET)

- [`SOURCE_SET_<NAME>`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCE_SET_NAME.html#prop_tgt:SOURCE_SET_%3CNAME%3E)

- [`SOURCE_DIRS`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCE_DIRS.html#prop_tgt:SOURCE_DIRS)

- [`SOURCE_DIRS_<NAME>`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCE_DIRS_NAME.html#prop_tgt:SOURCE_DIRS_%3CNAME%3E)

For file sets of type `CXX_MODULES`:

- [`CXX_MODULE_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_MODULE_SETS.html#prop_tgt:CXX_MODULE_SETS)

- [`INTERFACE_CXX_MODULE_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_CXX_MODULE_SETS.html#prop_tgt:INTERFACE_CXX_MODULE_SETS)

- [`CXX_MODULE_SET`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_MODULE_SET.html#prop_tgt:CXX_MODULE_SET)

- [`CXX_MODULE_SET_<NAME>`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_MODULE_SET_NAME.html#prop_tgt:CXX_MODULE_SET_%3CNAME%3E)

- [`CXX_MODULE_DIRS`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_MODULE_DIRS.html#prop_tgt:CXX_MODULE_DIRS)

- [`CXX_MODULE_DIRS_<NAME>`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_MODULE_DIRS_NAME.html#prop_tgt:CXX_MODULE_DIRS_%3CNAME%3E)

Target properties related to include directories are also modified by `target_sources(FILE_SET)` as follows:

[`INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INCLUDE_DIRECTORIES.html#prop_tgt:INCLUDE_DIRECTORIES)

If the `TYPE` is `HEADERS`, and the scope of the file set is `PRIVATE` or `PUBLIC`, all of the `BASE_DIRS` of the file set are wrapped in [`$<BUILD_INTERFACE>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:BUILD_INTERFACE) and appended to this property.

[`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES)

If the `TYPE` is `HEADERS`, and the scope of the file set is `INTERFACE` or `PUBLIC`, all of the `BASE_DIRS` of the file set are wrapped in [`$<BUILD_INTERFACE>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:BUILD_INTERFACE) and appended to this property.

## See Also

- [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable)

- [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library)

- [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions)

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
