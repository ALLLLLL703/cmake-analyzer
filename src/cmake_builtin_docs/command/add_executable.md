CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_executable.html)

# [add_executable](https://cmake.org/cmake/help/v4.4/command/add_executable.html#id1)

Contents

- [add_executable](https://cmake.org/cmake/help/v4.4/command/add_executable.html#add-executable)

   

  - [Normal Executables](https://cmake.org/cmake/help/v4.4/command/add_executable.html#normal-executables)

  - [Imported Executables](https://cmake.org/cmake/help/v4.4/command/add_executable.html#imported-executables)

  - [Alias Executables](https://cmake.org/cmake/help/v4.4/command/add_executable.html#alias-executables)

  - [See Also](https://cmake.org/cmake/help/v4.4/command/add_executable.html#see-also)

Add an executable to the project using the specified source files.

## [Normal Executables](https://cmake.org/cmake/help/v4.4/command/add_executable.html#id2)

**add_executable(\<name\> \<options\>... \<sources\>...)**

Add an [executable](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#executables) target called `<name>` to be built from the source files listed in the command invocation.

The options are:

**`WIN32`**

Set the [`WIN32_EXECUTABLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/WIN32_EXECUTABLE.html#prop_tgt:WIN32_EXECUTABLE) target property automatically. See documentation of that target property for details.

**`MACOSX_BUNDLE`**

Set the [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) target property automatically. See documentation of that target property for details.

**`EXCLUDE_FROM_ALL`**

Set the [`EXCLUDE_FROM_ALL`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXCLUDE_FROM_ALL.html#prop_tgt:EXCLUDE_FROM_ALL) target property automatically. See documentation of that target property for details.

The `<name>` corresponds to the logical target name and must be globally unique within a project. The actual file name of the executable built is constructed based on conventions of the native platform (such as `<name>.exe` or just `<name>`).

Added in version 3.1: Source arguments to `add_executable` may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

Added in version 3.11: The source files can be omitted if they are added later using [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources).

By default the executable file will be created in the build tree directory corresponding to the source tree directory in which the command was invoked. See documentation of the [`RUNTIME_OUTPUT_DIRECTORY`](https://cmake.org/cmake/help/v4.4/prop_tgt/RUNTIME_OUTPUT_DIRECTORY.html#prop_tgt:RUNTIME_OUTPUT_DIRECTORY) target property to change this location. See documentation of the [`OUTPUT_NAME`](https://cmake.org/cmake/help/v4.4/prop_tgt/OUTPUT_NAME.html#prop_tgt:OUTPUT_NAME) target property to change the `<name>` part of the final file name.

See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

See also [`HEADER_FILE_ONLY`](https://cmake.org/cmake/help/v4.4/prop_sf/HEADER_FILE_ONLY.html#prop_sf:HEADER_FILE_ONLY) on what to do if some sources are pre-processed, and you want to have the original sources reachable from within IDE.

## [Imported Executables](https://cmake.org/cmake/help/v4.4/command/add_executable.html#id3)

**add_executable(\<name\> IMPORTED [GLOBAL])**

Add an [IMPORTED executable target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets) to reference an executable file located outside the project. The target name may be referenced like any target built within the project, except that by default it is visible only in the directory in which it is created, and below.

The options are:

**`GLOBAL`**

Make the target name globally visible.

No rules are generated to build imported targets, and the [`IMPORTED`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED.html#prop_tgt:IMPORTED) target property is `True`. Imported executables are useful for convenient reference from commands like [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command).

Details about the imported executable are specified by setting properties whose names begin in `IMPORTED_`. The most important such property is [`IMPORTED_LOCATION`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_LOCATION.html#prop_tgt:IMPORTED_LOCATION) (and its per-configuration version [`IMPORTED_LOCATION_<CONFIG>`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_LOCATION_CONFIG.html#prop_tgt:IMPORTED_LOCATION_%3CCONFIG%3E)) which specifies the location of the main executable file on disk. See documentation of the `IMPORTED_*` properties for more information.

## [Alias Executables](https://cmake.org/cmake/help/v4.4/command/add_executable.html#id4)

**add_executable(\<name\> ALIAS \<target\>)**

Creates an [Alias Target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets), such that `<name>` can be used to refer to `<target>` in subsequent commands. The `<name>` does not appear in the generated buildsystem as a make target. The `<target>` may not be an `ALIAS`.

Added in version 3.11: An `ALIAS` can target a `GLOBAL` [Imported Target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets)

Added in version 3.18: An `ALIAS` can target a non-`GLOBAL` Imported Target. Such alias is scoped to the directory in which it is created and subdirectories. The [`ALIAS_GLOBAL`](https://cmake.org/cmake/help/v4.4/prop_tgt/ALIAS_GLOBAL.html#prop_tgt:ALIAS_GLOBAL) target property can be used to check if the alias is global or not.

`ALIAS` targets can be used as targets to read properties from, executables for custom commands and custom targets. They can also be tested for existence with the regular [`if(TARGET)`](https://cmake.org/cmake/help/v4.4/command/if.html#target) subcommand. The `<name>` may not be used to modify properties of `<target>`, that is, it may not be used as the operand of [`set_property()`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property), [`set_target_properties()`](https://cmake.org/cmake/help/v4.4/command/set_target_properties.html#command:set_target_properties), [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) etc. An `ALIAS` target may not be installed or exported.

## [See Also](https://cmake.org/cmake/help/v4.4/command/add_executable.html#id5)

- [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
