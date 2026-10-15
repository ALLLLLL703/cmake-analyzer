CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_library.html)

# [add_library](https://cmake.org/cmake/help/v4.4/command/add_library.html#id1)

Contents

- [add_library](https://cmake.org/cmake/help/v4.4/command/add_library.html#add-library)

   

  - [Normal Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#normal-libraries)

  - [Object Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#object-libraries)

  - [Interface Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#interface-libraries)

  - [Imported Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#imported-libraries)

  - [Alias Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#alias-libraries)

  - [See Also](https://cmake.org/cmake/help/v4.4/command/add_library.html#see-also)

Add a library to the project using the specified source files.

## [Normal Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#id2)

**add_library(\<name\> [\<type\>] [EXCLUDE_FROM_ALL] \<sources\>...)**

Add a library target called `<name>` to be built from the source files listed in the command invocation.

The optional `<type>` specifies the type of library to be created:

**`STATIC`**

A [Static Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#static-libraries): an archive of object files for use when linking other targets.

**`SHARED`**

A [Shared Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#shared-libraries): a dynamic library that may be linked by other targets and loaded at runtime.

**`MODULE`**

A [Module Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#module-libraries): a plugin that may not be linked by other targets, but may be dynamically loaded at runtime using dlopen-like functionality.

If no `<type>` is given the default is `STATIC` or `SHARED` based on the value of the [`BUILD_SHARED_LIBS`](https://cmake.org/cmake/help/v4.4/variable/BUILD_SHARED_LIBS.html#variable:BUILD_SHARED_LIBS) variable.

The options are:

**`EXCLUDE_FROM_ALL`**

Set the [`EXCLUDE_FROM_ALL`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXCLUDE_FROM_ALL.html#prop_tgt:EXCLUDE_FROM_ALL) target property automatically. See documentation of that target property for details.

The `<name>` corresponds to the logical target name and must be globally unique within a project. The actual file name of the library built is constructed based on conventions of the native platform (such as `lib<name>.a` or `<name>.lib`).

Added in version 3.1: Source arguments to `add_library` may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

Added in version 3.11: The source files can be omitted if they are added later using [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources).

For `SHARED` and `MODULE` libraries the [`POSITION_INDEPENDENT_CODE`](https://cmake.org/cmake/help/v4.4/prop_tgt/POSITION_INDEPENDENT_CODE.html#prop_tgt:POSITION_INDEPENDENT_CODE) target property is set to `ON` automatically. A `SHARED` library may be marked with the [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) target property to create an macOS Framework.

Added in version 3.8: A `STATIC` library may be marked with the [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) target property to create a static Framework.

If a library does not export any symbols, it must not be declared as a `SHARED` library. For example, a Windows resource DLL or a managed C++/CLI DLL that exports no unmanaged symbols would need to be a `MODULE` library. This is because CMake expects a `SHARED` library to always have an associated import library on Windows.

By default the library file will be created in the build tree directory corresponding to the source tree directory in which the command was invoked. See documentation of the [`ARCHIVE_OUTPUT_DIRECTORY`](https://cmake.org/cmake/help/v4.4/prop_tgt/ARCHIVE_OUTPUT_DIRECTORY.html#prop_tgt:ARCHIVE_OUTPUT_DIRECTORY), [`LIBRARY_OUTPUT_DIRECTORY`](https://cmake.org/cmake/help/v4.4/prop_tgt/LIBRARY_OUTPUT_DIRECTORY.html#prop_tgt:LIBRARY_OUTPUT_DIRECTORY), and [`RUNTIME_OUTPUT_DIRECTORY`](https://cmake.org/cmake/help/v4.4/prop_tgt/RUNTIME_OUTPUT_DIRECTORY.html#prop_tgt:RUNTIME_OUTPUT_DIRECTORY) target properties to change this location. See documentation of the [`OUTPUT_NAME`](https://cmake.org/cmake/help/v4.4/prop_tgt/OUTPUT_NAME.html#prop_tgt:OUTPUT_NAME) target property to change the `<name>` part of the final file name.

See the [`cmake-buildsystem(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#manual:cmake-buildsystem(7)) manual for more on defining buildsystem properties.

See also [`HEADER_FILE_ONLY`](https://cmake.org/cmake/help/v4.4/prop_sf/HEADER_FILE_ONLY.html#prop_sf:HEADER_FILE_ONLY) on what to do if some sources are pre-processed, and you want to have the original sources reachable from within IDE.

Changed in version 3.30: On platforms that do not support shared libraries, `add_library` now fails on calls creating `SHARED` libraries instead of automatically converting them to `STATIC` libraries as before. See policy [`CMP0164`](https://cmake.org/cmake/help/v4.4/policy/CMP0164.html#policy:CMP0164).

## [Object Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#id3)

**add_library(\<name\> OBJECT \<sources\>...)**

Add an [Object Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#object-libraries) to compile source files without archiving or linking their object files into a library.

Other targets created by `add_library` or [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) may reference the objects using an expression of the form [`$<TARGET_OBJECTS:objlib>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:TARGET_OBJECTS) as a source, where `objlib` is the object library name. For example:

~~~
add_library(... $<TARGET_OBJECTS:objlib> ...)
add_executable(... $<TARGET_OBJECTS:objlib> ...)
~~~

will include objlib's object files in a library and an executable along with those compiled from their own sources. Object libraries may contain only sources that compile, header files, and other files that would not affect linking of a normal library (e.g. `.txt`). They may contain custom commands generating such sources, but not `PRE_BUILD`, `PRE_LINK`, or `POST_BUILD` commands. Some native build systems (such as [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode)) may not like targets that have only object files, so consider adding at least one real source file to any target that references [`$<TARGET_OBJECTS:objlib>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:TARGET_OBJECTS).

Added in version 3.12: Object libraries can be linked to with [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries).

## [Interface Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#id4)

**add_library(\<name\> INTERFACE)**

Add an [Interface Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries) target that may specify usage requirements for dependents but does not compile sources and does not produce a library artifact on disk.

An interface library with no source files is not included as a target in the generated buildsystem. However, it may have properties set on it and it may be installed and exported. Typically, `INTERFACE_*` properties are populated on an interface target using the commands:

- [`set_property()`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property),

- [`target_link_libraries(INTERFACE)`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries),

- [`target_link_options(INTERFACE)`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options),

- [`target_include_directories(INTERFACE)`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories),

- [`target_compile_options(INTERFACE)`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options),

- [`target_compile_definitions(INTERFACE)`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions), and

- [`target_sources(INTERFACE)`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources),

and then it is used as an argument to [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) like any other target.

Added in version 3.15: An interface library can have [`PUBLIC_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PUBLIC_HEADER.html#prop_tgt:PUBLIC_HEADER) and [`PRIVATE_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRIVATE_HEADER.html#prop_tgt:PRIVATE_HEADER) properties. The headers specified by those properties can be installed using the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) command.

**add_library(\<name\> INTERFACE [EXCLUDE_FROM_ALL] \<sources\>...)**

Added in version 3.19.

Add an [Interface Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries) target with source files (in addition to usage requirements and properties as documented by the [`above signature`](https://cmake.org/cmake/help/v4.4/command/add_library.html#interface)). Source files may be listed directly in the `add_library` call or added later by calls to [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources) with the `PRIVATE` or `PUBLIC` keywords.

If an interface library has source files (i.e. the [`SOURCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOURCES.html#prop_tgt:SOURCES) target property is set), or header sets (i.e. the [`HEADER_SETS`](https://cmake.org/cmake/help/v4.4/prop_tgt/HEADER_SETS.html#prop_tgt:HEADER_SETS) target property is set), it will appear in the generated buildsystem as a build target much like a target defined by the [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) command. It does not compile any sources, but does contain build rules for custom commands created by the [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) command.

The options are:

**`EXCLUDE_FROM_ALL`**

Set the [`EXCLUDE_FROM_ALL`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXCLUDE_FROM_ALL.html#prop_tgt:EXCLUDE_FROM_ALL) target property automatically. See documentation of that target property for details.

Note

In most command signatures where the `INTERFACE` keyword appears, the items listed after it only become part of that target's usage requirements and are not part of the target's own settings. However, in this signature of `add_library`, the `INTERFACE` keyword refers to the library type only. Sources listed after it in the `add_library` call are `PRIVATE` to the interface library and do not appear in its [`INTERFACE_SOURCES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_SOURCES.html#prop_tgt:INTERFACE_SOURCES) target property.

**add_library(\<name\> INTERFACE SYMBOLIC)**

Added in version 4.2.

Add a symbolic [Interface Library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries) target. Symbolic interface libraries are useful for representing optional components or features in a package. They have no usage requirements, do not compile sources, and do not produce a library artifact on disk, but they may be exported and installed. They can also be tested for existence with the regular [`if(TARGET)`](https://cmake.org/cmake/help/v4.4/command/if.html#target) subcommand.

A symbolic interface library may be used as a linkable target to enforce the presence of optional components in a dependency. For example, if a library `libgui` may or may not provide a feature `widget`, a consumer package can link against `widget` to express that it requires this component to be available. This allows [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) calls that declare required components to be validated by linking against the corresponding symbolic targets.

A symbolic interface library has the [`SYMBOLIC`](https://cmake.org/cmake/help/v4.4/prop_tgt/SYMBOLIC.html#prop_tgt:SYMBOLIC) target property set to true.

## [Imported Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#id5)

**add_library(\<name\> \<type\> IMPORTED [GLOBAL])**

Add an [IMPORTED library target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets) called `<name>`. The target name may be referenced like any target built within the project, except that by default it is visible only in the directory in which it is created, and below.

The `<type>` must be one of:

**`STATIC`, `SHARED`, `MODULE`, `UNKNOWN`**

References a library file located outside the project. The [`IMPORTED_LOCATION`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_LOCATION.html#prop_tgt:IMPORTED_LOCATION) target property (or its per-configuration variant [`IMPORTED_LOCATION_<CONFIG>`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_LOCATION_CONFIG.html#prop_tgt:IMPORTED_LOCATION_%3CCONFIG%3E)) specifies the location of the main library file on disk:

- For a `SHARED` library on most non-Windows platforms, the main library file is the `.so` or `.dylib` file used by both linkers and dynamic loaders. If the referenced library file has a `SONAME` (or on macOS, has a `LC_ID_DYLIB` starting in `@rpath/`), the value of that field should be set in the [`IMPORTED_SONAME`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_SONAME.html#prop_tgt:IMPORTED_SONAME) target property. If the referenced library file does not have a `SONAME`, but the platform supports it, then the [`IMPORTED_NO_SONAME`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_NO_SONAME.html#prop_tgt:IMPORTED_NO_SONAME) target property should be set.

- For a `SHARED` library on Windows, the [`IMPORTED_IMPLIB`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_IMPLIB.html#prop_tgt:IMPORTED_IMPLIB) target property (or its per-configuration variant [`IMPORTED_IMPLIB_<CONFIG>`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_IMPLIB_CONFIG.html#prop_tgt:IMPORTED_IMPLIB_%3CCONFIG%3E)) specifies the location of the DLL import library file (`.lib` or `.dll.a`) on disk, and the `IMPORTED_LOCATION` is the location of the `.dll` runtime library (and is optional, but needed by the [`TARGET_RUNTIME_DLLS`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:TARGET_RUNTIME_DLLS) generator expression).

Additional usage requirements may be specified in `INTERFACE_*` properties.

An `UNKNOWN` library type is typically only used in the implementation of [Find Modules](https://cmake.org/cmake/help/v4.4/manual/cmake-developer.7.html#find-modules). It allows the path to an imported library (often found using the [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library) command) to be used without having to know what type of library it is. This is especially useful on Windows where a static library and a DLL's import library both have the same file extension.

**`OBJECT`**

References a set of object files located outside the project. The [`IMPORTED_OBJECTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_OBJECTS.html#prop_tgt:IMPORTED_OBJECTS) target property (or its per-configuration variant [`IMPORTED_OBJECTS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_OBJECTS_CONFIG.html#prop_tgt:IMPORTED_OBJECTS_%3CCONFIG%3E)) specifies the locations of object files on disk. Additional usage requirements may be specified in `INTERFACE_*` properties.

**`INTERFACE`**

Does not reference any library or object files on disk, but may specify usage requirements in `INTERFACE_*` properties.

The options are:

**`GLOBAL`**

Make the target name globally visible.

No rules are generated to build imported targets, and the [`IMPORTED`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED.html#prop_tgt:IMPORTED) target property is `True`. Imported libraries are useful for convenient reference from commands like [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries).

Details about the imported library are specified by setting properties whose names begin in `IMPORTED_` and `INTERFACE_`. See documentation of such properties for more information.

## [Alias Libraries](https://cmake.org/cmake/help/v4.4/command/add_library.html#id6)

**add_library(\<name\> ALIAS \<target\>)**

Creates an [Alias Target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets), such that `<name>` can be used to refer to `<target>` in subsequent commands. The `<name>` does not appear in the generated buildsystem as a make target. The `<target>` may not be an `ALIAS`.

Added in version 3.11: An `ALIAS` can target a `GLOBAL` [Imported Target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets)

Added in version 3.18: An `ALIAS` can target a non-`GLOBAL` Imported Target. Such alias is scoped to the directory in which it is created and below. The [`ALIAS_GLOBAL`](https://cmake.org/cmake/help/v4.4/prop_tgt/ALIAS_GLOBAL.html#prop_tgt:ALIAS_GLOBAL) target property can be used to check if the alias is global or not.

`ALIAS` targets can be used as linkable targets and as targets to read properties from. They can also be tested for existence with the regular [`if(TARGET)`](https://cmake.org/cmake/help/v4.4/command/if.html#target) subcommand. The `<name>` may not be used to modify properties of `<target>`, that is, it may not be used as the operand of [`set_property()`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property), [`set_target_properties()`](https://cmake.org/cmake/help/v4.4/command/set_target_properties.html#command:set_target_properties), [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) etc. An `ALIAS` target may not be installed or exported.

## [See Also](https://cmake.org/cmake/help/v4.4/command/add_library.html#id7)

- [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
