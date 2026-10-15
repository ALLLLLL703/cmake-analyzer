CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/install.html)

# [install](https://cmake.org/cmake/help/v4.4/command/install.html#id1)

Contents

- [install](https://cmake.org/cmake/help/v4.4/command/install.html#install)

   

  - [Synopsis](https://cmake.org/cmake/help/v4.4/command/install.html#synopsis)

  - [Introduction](https://cmake.org/cmake/help/v4.4/command/install.html#introduction)

     

    - [Common Options](https://cmake.org/cmake/help/v4.4/command/install.html#common-options)

  - [Signatures](https://cmake.org/cmake/help/v4.4/command/install.html#signatures)

  - [Examples](https://cmake.org/cmake/help/v4.4/command/install.html#examples)

     

    - [Example: Install Targets with Per-Artifact Components](https://cmake.org/cmake/help/v4.4/command/install.html#example-install-targets-with-per-artifact-components)

    - [Example: Install Targets to Per-Config Destinations](https://cmake.org/cmake/help/v4.4/command/install.html#example-install-targets-to-per-config-destinations)

  - [Generated Installation Script](https://cmake.org/cmake/help/v4.4/command/install.html#generated-installation-script)

Specify rules to run at install time.

## [Synopsis](https://cmake.org/cmake/help/v4.4/command/install.html#id2)

~~~
install(TARGETS <target>... [...])
install(IMPORTED_RUNTIME_ARTIFACTS <target>... [...])
install({FILES | PROGRAMS} <file>... [...])
install(DIRECTORY <dir>... [...])
install(SCRIPT <file> [...])
install(CODE <code> [...])
install(EXPORT <export-name> [...])
install(PACKAGE_INFO <package-name> [...])
install(RUNTIME_DEPENDENCY_SET <set-name> [...])
install(SBOM <sbom-name> [...])
~~~

## [Introduction](https://cmake.org/cmake/help/v4.4/command/install.html#id3)

This command generates installation rules for a project. Install rules specified by calls to the `install()` command within a source directory are executed in order during installation.

Changed in version 3.14: Install rules in subdirectories added by calls to the [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) command are interleaved with those in the parent directory to run in the order declared (see policy [`CMP0082`](https://cmake.org/cmake/help/v4.4/policy/CMP0082.html#policy:CMP0082)).

Changed in version 3.22: The environment variable [`CMAKE_INSTALL_MODE`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_INSTALL_MODE.html#envvar:CMAKE_INSTALL_MODE) can override the default copying behavior of `install()`.

Changed in version 3.31: Projects can enable [`INSTALL_PARALLEL`](https://cmake.org/cmake/help/v4.4/prop_gbl/INSTALL_PARALLEL.html#prop_gbl:INSTALL_PARALLEL) to enable a parallel installation. When using the parallel install, subdirectories added by calls to the [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) command are installed independently and the order that install rules added in different subdirectories will run is not guaranteed.

### [Common Options](https://cmake.org/cmake/help/v4.4/command/install.html#id4)

There are multiple signatures for this command. Some of them define installation options for files and targets. Options common to multiple signatures are covered here but they are valid only for signatures that specify them. The common options are:

**`DESTINATION <dir>`**

Specify the directory on disk to which a file will be installed. `<dir>` should be a relative path. An absolute path is allowed, but not recommended.

Added in version 4.4: The [`CMD_INSTALL_ABSOLUTE_DESTINATION`](https://cmake.org/cmake/help/v4.4/diagnostic/CMD_INSTALL_ABSOLUTE_DESTINATION.html#diagnostic:CMD_INSTALL_ABSOLUTE_DESTINATION) diagnostic can be enabled to warn or error out when an absolute destination is provided.

When a relative path is given, it is interpreted relative to the value of the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) variable. The prefix can be relocated at install time using the `DESTDIR` mechanism explained in the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX) variable documentation.

As absolute paths do not work with the `cmake --install` command's [`--prefix`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake--install-0) option, or with the [`cpack`](https://cmake.org/cmake/help/v4.4/manual/cpack.1.html#manual:cpack(1)) installer generators, it is strongly recommended to use relative paths throughout for best support by package maintainers. In particular, there is no need to make paths absolute by prepending [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX); this prefix is used by default if the DESTINATION is a relative path.

If an absolute path (with a leading slash or drive letter) is given it is used verbatim.

Changed in version 3.31: `<dir>` will be normalized according to the same [normalization rules](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#normalization) as the [`cmake_path()`](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#command:cmake_path) command.

**`PERMISSIONS <permission>...`**

Specify permissions for installed files. Valid permissions are `OWNER_READ`, `OWNER_WRITE`, `OWNER_EXECUTE`, `GROUP_READ`, `GROUP_WRITE`, `GROUP_EXECUTE`, `WORLD_READ`, `WORLD_WRITE`, `WORLD_EXECUTE`, `SETUID`, and `SETGID`. Permissions that do not make sense on certain platforms are ignored on those platforms.

If this option is used multiple times in a single call, its list of permissions accumulates. If an [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) call uses [\<artifact-kind\>](https://cmake.org/cmake/help/v4.4/command/install.html#artifact-kind) arguments, a separate list of permissions is accumulated for each kind of artifact.

**`CONFIGURATIONS <config>...`**

Specify a list of build configurations for which the install rule applies (Debug, Release, etc.).

If this option is used multiple times in a single call, its list of configurations accumulates. If an [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) call uses [\<artifact-kind\>](https://cmake.org/cmake/help/v4.4/command/install.html#artifact-kind) arguments, a separate list of configurations is accumulated for each kind of artifact.

**`COMPONENT <component>`**

Specify an installation component name with which the install rule is associated, such as `Runtime` or `Development`. During component-specific installation only install rules associated with the given component name will be executed. During a full installation all components are installed unless marked with `EXCLUDE_FROM_ALL`. If `COMPONENT` is not provided a default component "Unspecified" is created. The default component name may be controlled with the [`CMAKE_INSTALL_DEFAULT_COMPONENT_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_DEFAULT_COMPONENT_NAME.html#variable:CMAKE_INSTALL_DEFAULT_COMPONENT_NAME) variable.

Installation components can be then used by the `cmake --install` command's [`--component`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-install-component) option and the [`CPackComponent`](https://cmake.org/cmake/help/v4.4/module/CPackComponent.html#module:CPackComponent) module. Global target `list_install_components` lists all available components.

**`EXCLUDE_FROM_ALL`**

Added in version 3.6.

Specify that the file is excluded from a full installation and only installed as part of a component-specific installation

**`OPTIONAL`**

Specify that it is not an error if the file to be installed does not exist.

Added in version 3.1: Command signatures that install files may print messages during installation. Use the [`CMAKE_INSTALL_MESSAGE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_MESSAGE.html#variable:CMAKE_INSTALL_MESSAGE) variable to control which messages are printed.

Added in version 3.11: Many of the `install()` variants implicitly create the directories containing the installed files. If [`CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS.html#variable:CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS) is set, these directories will be created with the permissions specified. Otherwise, they will be created according to the uname rules on Unix-like platforms. Windows platforms are unaffected.

## [Signatures](https://cmake.org/cmake/help/v4.4/command/install.html#id5)

**install(TARGETS \<target\>... [...])**

Install target [Output Artifacts](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#output-artifacts) and associated files:

~~~
install(TARGETS <target>... [EXPORT <export-name>]
        [RUNTIME_DEPENDENCIES <arg>...|RUNTIME_DEPENDENCY_SET <set-name>]
        [<artifact-option>...]
        [<artifact-kind> <artifact-option>...]...
        [INCLUDES DESTINATION [<dir> ...]]
        )
~~~

where `<artifact-option>...` group may contain:

~~~
[DESTINATION <dir>]
[PERMISSIONS <permission>...]
[CONFIGURATIONS <config>...]
[COMPONENT <component>]
[NAMELINK_COMPONENT <component>]
[OPTIONAL] [EXCLUDE_FROM_ALL]
[NAMELINK_ONLY|NAMELINK_SKIP]
~~~

The first `<artifact-option>...` group applies to target [Output Artifacts](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#output-artifacts) that do not have a dedicated group specified later in the same call.

Each `<artifact-kind> <artifact-option>...` group applies to [Output Artifacts](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#output-artifacts) of the specified artifact kind:

**`ARCHIVE`**

Target artifacts of this kind include:

- *Static libraries* (except on macOS when marked as `FRAMEWORK`, see below);

- *DLL import libraries* (on all Windows-based systems including Cygwin; they have extension `.lib`, in contrast to the `.dll` libraries that go to `RUNTIME`);

- On AIX, the *linker import file* created for executables with [`ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/ENABLE_EXPORTS.html#prop_tgt:ENABLE_EXPORTS) enabled.

- On macOS, the *linker import file* created for shared libraries with [`ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/ENABLE_EXPORTS.html#prop_tgt:ENABLE_EXPORTS) enabled (except when marked as `FRAMEWORK`, see below).

**`LIBRARY`**

Target artifacts of this kind include:

- *Shared libraries*, except

   

  - DLLs (these go to `RUNTIME`, see below),

  - on macOS when marked as `FRAMEWORK` (see below).

- *Module libraries*

**`RUNTIME`**

Target artifacts of this kind include:

- *Executables* (except on macOS when marked as `MACOSX_BUNDLE`, see `BUNDLE` below);

- DLLs (on all Windows-based systems including Cygwin; note that the accompanying import libraries are of kind `ARCHIVE`).

**`OBJECTS`**

Added in version 3.9.

Object files associated with *object libraries*.

**`FRAMEWORK`**

Both static and shared libraries marked with the `FRAMEWORK` property are treated as `FRAMEWORK` targets on macOS.

**`BUNDLE`**

Executables marked with the [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) property are treated as `BUNDLE` targets on macOS.

**`PUBLIC_HEADER`**

Any [`PUBLIC_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PUBLIC_HEADER.html#prop_tgt:PUBLIC_HEADER) files associated with a library are installed in the destination specified by the `PUBLIC_HEADER` argument on non-Apple platforms. Rules defined by this argument are ignored for [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) libraries on Apple platforms because the associated files are installed into the appropriate locations inside the framework folder. See [`PUBLIC_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PUBLIC_HEADER.html#prop_tgt:PUBLIC_HEADER) for details.

**`PRIVATE_HEADER`**

Similar to `PUBLIC_HEADER`, but for `PRIVATE_HEADER` files. See [`PRIVATE_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRIVATE_HEADER.html#prop_tgt:PRIVATE_HEADER) for details.

**`RESOURCE`**

Similar to `PUBLIC_HEADER` and `PRIVATE_HEADER`, but for `RESOURCE` files. See [`RESOURCE`](https://cmake.org/cmake/help/v4.4/prop_tgt/RESOURCE.html#prop_tgt:RESOURCE) for details.

**`FILE_SET <set-name>`**

Added in version 3.23.

File sets are defined by the [`target_sources(FILE_SET)`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources) command. If the file set `<set-name>` exists and is `PUBLIC` or `INTERFACE`, any files in the set of type `HEADERS` are installed under the destination (see below). Other types do not have any default destination, so `DESTINATION` option must be specified for each `FILE_SET`. The directory structure relative to the file set's base directories is preserved. For example, a file added to the file set as `/blah/include/myproj/here.h` with a base directory `/blah/include` would be installed to `myproj/here.h` below the destination.

**`CXX_MODULES_BMI`**

Added in version 3.28.

Any module files from C++ modules from `PUBLIC` sources in a file set of type `CXX_MODULES` will be installed to the given `DESTINATION`. All modules are placed directly in the destination as no directory structure is derived from the names of the modules. An empty `DESTINATION` may be used to suppress installing these files (for use in generic code).

For regular executables, static libraries and shared libraries, the `DESTINATION` argument is not required. For these target types, when `DESTINATION` is omitted, a default destination will be taken from the appropriate variable from [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs), or set to a built-in default value if that variable is not defined. The same is true for file sets of type `HEADERS`, and the public and private headers associated with the installed targets through the [`PUBLIC_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PUBLIC_HEADER.html#prop_tgt:PUBLIC_HEADER) and [`PRIVATE_HEADER`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRIVATE_HEADER.html#prop_tgt:PRIVATE_HEADER) target properties. A destination must always be provided for module libraries, Apple bundles and frameworks. A destination can be omitted for interface and object libraries, but they are handled differently (see the discussion of this topic toward the end of this section).

For shared libraries on DLL platforms, if neither `RUNTIME` nor `ARCHIVE` destinations are specified, both the `RUNTIME` and `ARCHIVE` components are installed to their default destinations. If either a `RUNTIME` or `ARCHIVE` destination is specified, the component is installed to that destination, and the other component is not installed. If both `RUNTIME` and `ARCHIVE` destinations are specified, then both components are installed to their respective destinations.

The following table shows the target types with their associated variables and built-in defaults that apply when no destination is given:

Target Type |  GNUInstallDirs Variable |  Built-In Default |

`RUNTIME` |  `${CMAKE_INSTALL_BINDIR}` |  `bin` |

`LIBRARY` |  `${CMAKE_INSTALL_LIBDIR}` |  `lib` |

`ARCHIVE` |  `${CMAKE_INSTALL_LIBDIR}` |  `lib` |

`PRIVATE_HEADER` |  `${CMAKE_INSTALL_INCLUDEDIR}` |  `include` |

`PUBLIC_HEADER` |  `${CMAKE_INSTALL_INCLUDEDIR}` |  `include` |

`FILE_SET` (type `HEADERS`) |  `${CMAKE_INSTALL_INCLUDEDIR}` |  `include` |

Projects wishing to follow the common practice of installing headers into a project-specific subdirectory may prefer using file sets with appropriate paths and base directories. Otherwise, they must provide a `DESTINATION` instead of being able to rely on the above (see next example below).

To make packages compliant with distribution filesystem layout policies, if projects must specify a `DESTINATION`, it is strongly recommended that they use a path that begins with the appropriate relative [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs) variable. This allows package maintainers to control the install destination by setting the appropriate cache variables. The following example shows a static library being installed to the default destination provided by [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs), but with its headers installed to a project-specific subdirectory without using file sets:

~~~
add_library(mylib STATIC ...)
set_target_properties(mylib PROPERTIES PUBLIC_HEADER mylib.h)
include(GNUInstallDirs)
install(TARGETS mylib
        PUBLIC_HEADER
          DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/myproj
)
~~~

In addition to the [common options](https://cmake.org/cmake/help/v4.4/command/install.html#common-options) listed above, each target can accept the following additional arguments:

**`NAMELINK_COMPONENT`**

Added in version 3.12.

On some platforms a versioned shared library has a symbolic link such as:

~~~
lib<name>.so -> lib<name>.so.1
~~~

where `lib<name>.so.1` is the soname of the library and `lib<name>.so` is a "namelink" allowing linkers to find the library when given `-l<name>`. The `NAMELINK_COMPONENT` option is similar to the `COMPONENT` option, but it changes the installation component of a shared library namelink if one is generated. If not specified, this defaults to the value of `COMPONENT`. It is an error to use this parameter outside of a `LIBRARY` block.

Changed in version 3.27: This parameter is also usable for an `ARCHIVE` block to manage the linker import file created, on macOS, for shared libraries with [`ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/ENABLE_EXPORTS.html#prop_tgt:ENABLE_EXPORTS) enabled.

See the [Example: Install Targets with Per-Artifact Components](https://cmake.org/cmake/help/v4.4/command/install.html#example-install-targets-with-per-artifact-components) for an example using `NAMELINK_COMPONENT`.

This option is typically used for package managers that have separate runtime and development packages. For example, on Debian systems, the library is expected to be in the runtime package, and the headers and namelink are expected to be in the development package.

See the [`VERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/VERSION.html#prop_tgt:VERSION) and [`SOVERSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/SOVERSION.html#prop_tgt:SOVERSION) target properties for details on creating versioned shared libraries.

**`NAMELINK_ONLY`**

This option causes the installation of only the namelink when a library target is installed. On platforms where versioned shared libraries do not have namelinks or when a library is not versioned, the `NAMELINK_ONLY` option installs nothing. It is an error to use this parameter outside of a `LIBRARY` block.

Changed in version 3.27: This parameter is also usable for an `ARCHIVE` block to manage the linker import file created, on macOS, for shared libraries with [`ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/ENABLE_EXPORTS.html#prop_tgt:ENABLE_EXPORTS) enabled.

When `NAMELINK_ONLY` is given, either `NAMELINK_COMPONENT` or `COMPONENT` may be used to specify the installation component of the namelink, but `COMPONENT` should generally be preferred.

**`NAMELINK_SKIP`**

Similar to `NAMELINK_ONLY`, but it has the opposite effect: it causes the installation of library files other than the namelink when a library target is installed. When neither `NAMELINK_ONLY` or `NAMELINK_SKIP` are given, both portions are installed. On platforms where versioned shared libraries do not have symlinks or when a library is not versioned, `NAMELINK_SKIP` installs the library. It is an error to use this parameter outside of a `LIBRARY` block.

Changed in version 3.27: This parameter is also usable for an `ARCHIVE` block to manage the linker import file created, on macOS, for shared libraries with [`ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/ENABLE_EXPORTS.html#prop_tgt:ENABLE_EXPORTS) enabled.

If `NAMELINK_SKIP` is specified, `NAMELINK_COMPONENT` has no effect. It is not recommended to use `NAMELINK_SKIP` in conjunction with `NAMELINK_COMPONENT`.

The [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) command can also accept the following options at the top level:

**`EXPORT`**

This option associates the installed target files with an export called `<export-name>`. It must appear before any target options. To actually install the export file itself, call [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export), documented below. See documentation of the [`EXPORT_NAME`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXPORT_NAME.html#prop_tgt:EXPORT_NAME) target property to change the name of the exported target.

If `EXPORT` is used and the targets include `PUBLIC` or `INTERFACE` file sets, all of them must be specified with `FILE_SET` arguments. All `PUBLIC` or `INTERFACE` file sets associated with a target are included in the export.

**`INCLUDES DESTINATION`**

This option specifies a list of directories which will be added to the [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES) target property of the `<targets>` when exported by the [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) command. If a relative path is specified, it is treated as relative to the [`$<INSTALL_PREFIX>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:INSTALL_PREFIX).

Unlike other `DESTINATION` arguments for the various `install()` subcommands, paths given after `INCLUDES DESTINATION` are used as given. They are not normalized, nor assumed to be normalized, although it is recommended that they are given in normalized form (see [Normalization](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#normalization)).

**`RUNTIME_DEPENDENCY_SET <set-name>`**

Added in version 3.21.

This option causes all runtime dependencies of installed executable, shared library, and module targets to be added to the specified runtime dependency set. This set can then be installed with an [`install(RUNTIME_DEPENDENCY_SET)`](https://cmake.org/cmake/help/v4.4/command/install.html#runtime-dependency-set) command.

This keyword and the `RUNTIME_DEPENDENCIES` keyword are mutually exclusive.

**`RUNTIME_DEPENDENCIES <arg>...`**

Added in version 3.21.

This option causes all runtime dependencies of installed executable, shared library, and module targets to be installed along with the targets themselves. The `RUNTIME`, `LIBRARY`, `FRAMEWORK`, and generic arguments are used to determine the properties (`DESTINATION`, `COMPONENT`, etc.) of the installation of these dependencies.

`RUNTIME_DEPENDENCIES` is semantically equivalent to the following pair of calls:

~~~
install(TARGETS ... RUNTIME_DEPENDENCY_SET <set-name>)
install(RUNTIME_DEPENDENCY_SET <set-name> <arg>...)
~~~

where `<set-name>` will be a randomly generated set name. `<arg>...` may include any of the following keywords supported by the [`install(RUNTIME_DEPENDENCY_SET)`](https://cmake.org/cmake/help/v4.4/command/install.html#runtime-dependency-set) command:

- `DIRECTORIES`

- `PRE_INCLUDE_REGEXES`

- `PRE_EXCLUDE_REGEXES`

- `POST_INCLUDE_REGEXES`

- `POST_EXCLUDE_REGEXES`

- `POST_INCLUDE_FILES`

- `POST_EXCLUDE_FILES`

The `RUNTIME_DEPENDENCIES` and `RUNTIME_DEPENDENCY_SET` keywords are mutually exclusive.

[Interface Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries) may be listed among the targets to install. They install no artifacts but will be included in an associated `EXPORT`. If [Object Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#object-libraries) are listed but given no destination for their object files, they will be exported as [Interface Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries). This is sufficient to satisfy transitive usage requirements of other targets that link to the object libraries in their implementation.

Installing a target with the [`EXCLUDE_FROM_ALL`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXCLUDE_FROM_ALL.html#prop_tgt:EXCLUDE_FROM_ALL) target property set to `TRUE` has undefined behavior.

Added in version 3.3: An install destination given as a `DESTINATION` argument may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

Added in version 3.13: [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) can install targets that were created in other directories. When using such cross-directory install rules, running `make install` (or similar) from a subdirectory will not guarantee that targets from other directories are up-to-date. You can use [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries) or [`add_dependencies()`](https://cmake.org/cmake/help/v4.4/command/add_dependencies.html#command:add_dependencies) to ensure that such out-of-directory targets are built before the subdirectory-specific install rules are run.

**install(IMPORTED_RUNTIME_ARTIFACTS \<target\>... [...])**

Added in version 3.21.

Install runtime artifacts of imported targets:

~~~
install(IMPORTED_RUNTIME_ARTIFACTS <target>...
        [RUNTIME_DEPENDENCY_SET <set-name>]
        [[LIBRARY|RUNTIME|FRAMEWORK|BUNDLE]
         [DESTINATION <dir>]
         [PERMISSIONS <permission>...]
         [CONFIGURATIONS <config>...]
         [COMPONENT <component>]
         [OPTIONAL] [EXCLUDE_FROM_ALL]
        ]...
        )
~~~

The `IMPORTED_RUNTIME_ARTIFACTS` form specifies rules for installing the runtime artifacts of imported targets. Projects may do this if they want to bundle outside executables or modules inside their installation. The `LIBRARY`, `RUNTIME`, `FRAMEWORK`, and `BUNDLE` arguments have the same semantics that they do in the [TARGETS](https://cmake.org/cmake/help/v4.4/command/install.html#targets) mode. Only the runtime artifacts of imported targets are installed (except in the case of [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) libraries, [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) executables, and [`BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/BUNDLE.html#prop_tgt:BUNDLE) CFBundles.) For example, headers and import libraries associated with DLLs are not installed. In the case of [`FRAMEWORK`](https://cmake.org/cmake/help/v4.4/prop_tgt/FRAMEWORK.html#prop_tgt:FRAMEWORK) libraries, [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) executables, and [`BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/BUNDLE.html#prop_tgt:BUNDLE) CFBundles, the entire directory is installed.

The `RUNTIME_DEPENDENCY_SET` option causes the runtime artifacts of the imported executable, shared library, and module library `targets` to be added to the `<set-name>` runtime dependency set. This set can then be installed with an [`install(RUNTIME_DEPENDENCY_SET)`](https://cmake.org/cmake/help/v4.4/command/install.html#runtime-dependency-set) command.

**install(FILES \<file\>... [...])**

**install(PROGRAMS \<program\>... [...])**

Note

If installing header files, consider using file sets defined by [`target_sources(FILE_SET)`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources) instead. File sets associate headers with a target and they install as part of the target.

Install files or programs:

~~~
install(<FILES|PROGRAMS> <file>...
        TYPE <type> | DESTINATION <dir>
        [PERMISSIONS <permission>...]
        [CONFIGURATIONS <config>...]
        [COMPONENT <component>]
        [RENAME <name>] [OPTIONAL] [EXCLUDE_FROM_ALL])
~~~

The `FILES` form specifies rules for installing files for a project. File names given as relative paths are interpreted with respect to the current source directory. Files installed by this form are by default given permissions `OWNER_WRITE`, `OWNER_READ`, `GROUP_READ`, and `WORLD_READ` if no `PERMISSIONS` argument is given.

The `PROGRAMS` form is identical to the `FILES` form except that the default permissions for the installed file also include `OWNER_EXECUTE`, `GROUP_EXECUTE`, and `WORLD_EXECUTE`. This form is intended to install programs that are not targets, such as shell scripts. Use the `TARGETS` form to install targets built within the project.

The list of files given to `FILES` or `PROGRAMS` may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. However, if any item begins in a generator expression it must evaluate to a full path.

The optional `RENAME <name>` argument is used to specify a name for the installed file that is different from the original file name. Renaming is allowed only when a single file is installed by the command.

Either a `TYPE` or a `DESTINATION` must be provided, but not both. A `TYPE` argument specifies the generic file type of the files being installed. A destination will then be set automatically by taking the corresponding variable from [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs), or by using a built-in default if that variable is not defined. See the table below for the supported file types and their corresponding variables and built-in defaults. Projects can provide a `DESTINATION` argument instead of a file type if they wish to explicitly define the install destination.

`TYPE` Argument |  GNUInstallDirs Variable |  Built-In Default |

`BIN` |  `${CMAKE_INSTALL_BINDIR}` |  `bin` |

`SBIN` |  `${CMAKE_INSTALL_SBINDIR}` |  `sbin` |

`LIB` |  `${CMAKE_INSTALL_LIBDIR}` |  `lib` |

`INCLUDE` |  `${CMAKE_INSTALL_INCLUDEDIR}` |  `include` |

`SYSCONF` |  `${CMAKE_INSTALL_SYSCONFDIR}` |  `etc` |

`SHAREDSTATE` |  `${CMAKE_INSTALL_SHARESTATEDIR}` |  `com` |

`LOCALSTATE` |  `${CMAKE_INSTALL_LOCALSTATEDIR}` |  `var` |

`RUNSTATE` |  `${CMAKE_INSTALL_RUNSTATEDIR}` |  `<LOCALSTATE dir>/run` |

`DATA` |  `${CMAKE_INSTALL_DATADIR}` |  `<DATAROOT dir>` |

`INFO` |  `${CMAKE_INSTALL_INFODIR}` |  `<DATAROOT dir>/info` |

`LOCALE` |  `${CMAKE_INSTALL_LOCALEDIR}` |  `<DATAROOT dir>/locale` |

`MAN` |  `${CMAKE_INSTALL_MANDIR}` |  `<DATAROOT dir>/man` |

`DOC` |  `${CMAKE_INSTALL_DOCDIR}` |  `<DATAROOT dir>/doc` |

`LIBEXEC` |  `${CMAKE_INSTALL_LIBEXECDIR}` |  `libexec` |

Projects wishing to follow the common practice of installing headers into a project-specific subdirectory will need to provide a destination rather than rely on the above. Using file sets for headers instead of `install(FILES)` would be even better (see [`target_sources(FILE_SET)`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)).

Note that some of the types' built-in defaults use the `DATAROOT` directory as a prefix. The `DATAROOT` prefix is calculated similarly to the types, with `CMAKE_INSTALL_DATAROOTDIR` as the variable and `share` as the built-in default. You cannot use `DATAROOT` as a `TYPE` parameter; please use `DATA` instead.

To make packages compliant with distribution filesystem layout policies, if projects must specify a `DESTINATION`, it is strongly recommended that they use a path that begins with the appropriate relative [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs) variable. This allows package maintainers to control the install destination by setting the appropriate cache variables. The following example shows how to follow this advice while installing an image to a project-specific documentation subdirectory:

~~~
include(GNUInstallDirs)
install(FILES logo.png
        DESTINATION ${CMAKE_INSTALL_DOCDIR}/myproj
)
~~~

Added in version 3.4: An install destination given as a `DESTINATION` argument may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

Added in version 3.20: An install rename given as a `RENAME` argument may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

Added in version 3.31: The `TYPE` argument now supports type `LIBEXEC`.

**install(DIRECTORY \<dir\>... [...])**

Note

To install a directory sub-tree of headers, consider using file sets defined by [`target_sources(FILE_SET)`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources) instead. File sets not only preserve directory structure, they also associate headers with a target and install as part of the target.

Install the contents of one or more directories:

~~~
install(DIRECTORY <dir>...
        TYPE <type> | DESTINATION <dir>
        [FILE_PERMISSIONS <permission>...]
        [DIRECTORY_PERMISSIONS <permission>...]
        [USE_SOURCE_PERMISSIONS] [OPTIONAL] [MESSAGE_NEVER]
        [CONFIGURATIONS <config>...]
        [COMPONENT <component>] [EXCLUDE_FROM_ALL]
        [FILES_MATCHING]
        [<match-rule> <match-option>...]...
        )
~~~

The `DIRECTORY` form installs contents of one or more directories to a given destination. The directory structure is copied verbatim to the destination.

Either a `TYPE` or a `DESTINATION` must be provided, but not both. If no permissions is given, files will be given the default permissions specified in the [FILES](https://cmake.org/cmake/help/v4.4/command/install.html#files) form of the command, and the directories will be given the default permissions specified in the [PROGRAMS](https://cmake.org/cmake/help/v4.4/command/install.html#programs) form of the command.

The options are:

**`<dir>...`**

The list of directories to be installed.

The last component of each directory name is appended to the destination directory but a trailing slash may be used to avoid this because it leaves the last component empty. Directory names given as relative paths are interpreted with respect to the current source directory. If no input directory names are given the destination directory will be created but nothing will be installed into it.

Added in version 3.5: The source `<dir>...` list may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

**`TYPE <type>`**

Specifies the generic file type of the files within the listed directories being installed. A destination will then be set automatically by taking the corresponding variable from [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs), or by using a built-in default if that variable is not defined. See the table below for the supported file types and their corresponding variables and built-in defaults. Projects can provide a `DESTINATION` argument instead of a file type if they wish to explicitly define the install destination.

`TYPE` Argument |  GNUInstallDirs Variable |  Built-In Default |

`BIN` |  `${CMAKE_INSTALL_BINDIR}` |  `bin` |

`SBIN` |  `${CMAKE_INSTALL_SBINDIR}` |  `sbin` |

`LIB` |  `${CMAKE_INSTALL_LIBDIR}` |  `lib` |

`INCLUDE` |  `${CMAKE_INSTALL_INCLUDEDIR}` |  `include` |

`SYSCONF` |  `${CMAKE_INSTALL_SYSCONFDIR}` |  `etc` |

`SHAREDSTATE` |  `${CMAKE_INSTALL_SHARESTATEDIR}` |  `com` |

`LOCALSTATE` |  `${CMAKE_INSTALL_LOCALSTATEDIR}` |  `var` |

`RUNSTATE` |  `${CMAKE_INSTALL_RUNSTATEDIR}` |  `<LOCALSTATE dir>/run` |

`DATA` |  `${CMAKE_INSTALL_DATADIR}` |  `<DATAROOT dir>` |

`INFO` |  `${CMAKE_INSTALL_INFODIR}` |  `<DATAROOT dir>/info` |

`LOCALE` |  `${CMAKE_INSTALL_LOCALEDIR}` |  `<DATAROOT dir>/locale` |

`MAN` |  `${CMAKE_INSTALL_MANDIR}` |  `<DATAROOT dir>/man` |

`DOC` |  `${CMAKE_INSTALL_DOCDIR}` |  `<DATAROOT dir>/doc` |

`LIBEXEC` |  `${CMAKE_INSTALL_LIBEXECDIR}` |  `libexec` |

Note that some of the types' built-in defaults use the `DATAROOT` directory as a prefix. The `DATAROOT` prefix is calculated similarly to the types, with `CMAKE_INSTALL_DATAROOTDIR` as the variable and `share` as the built-in default. You cannot use `DATAROOT` as a `TYPE` parameter; please use `DATA` instead.

Added in version 3.31: The `LIBEXEC` type.

**`DESTINATION <dir>`**

The destination directory, as documented among the [common options](https://cmake.org/cmake/help/v4.4/command/install.html#common-options).

To make packages compliant with distribution filesystem layout policies, if projects must specify a `DESTINATION`, it is strongly recommended that they use a path that begins with the appropriate relative [`GNUInstallDirs`](https://cmake.org/cmake/help/v4.4/module/GNUInstallDirs.html#module:GNUInstallDirs) variable. This allows package maintainers to control the install destination by setting the appropriate cache variables.

Added in version 3.4: The destination `<dir>` may use "generator expressions" with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

**`FILE_PERMISSIONS <permission>...`**

Specify permissions given to files in the destination, where the `<permission>` names are documented among the [common options](https://cmake.org/cmake/help/v4.4/command/install.html#common-options).

**`DIRECTORY_PERMISSIONS <permission>...`**

Specify permissions given to directories in the destination, where the `<permission>` names are documented among the [common options](https://cmake.org/cmake/help/v4.4/command/install.html#common-options).

**`USE_SOURCE_PERMISSIONS`**

If specified, and `FILE_PERMISSIONS` is not, file permissions will be copied from the source directory structure.

**`MESSAGE_NEVER`**

Added in version 3.1.

Disable file installation status output.

**`FILES_MATCHING`**

This option may be given before the first `<match-rule>` to disable installation of files (but not directories) not matched by any expression. For example, the code

~~~
install(DIRECTORY src/ DESTINATION doc/myproj
        FILES_MATCHING PATTERN "*.png")
~~~

will extract and install images from a source tree.

**`<match-rule> <match-option>...`**

Installation of directories may be controlled with fine granularity using rules that match directories or files encountered within input directories. They may be used to apply certain options (see below) to a subset of the files and directories encountered. All files and directories are installed whether or not they are matched, unless the above `FILES_MATCHING` option is given.

The full path to each input file or directory, with forward slashes, may be matched by a `<match-rule>`:

**`PATTERN <pattern>`**

Match complete file names using a globbing pattern. The portion of the full path matching the pattern must occur at the end of the file name and be preceded by a slash (which is not part of the pattern).

**`REGEX <regex>`**

Match any portion of the full path of a file with a [regular expression](https://cmake.org/cmake/help/v4.4/command/string.html#regex-specification). One may use `/` and `$` to limit matching to the end of a path.

Each `<match-rule>` may be followed by `<match-option>` arguments. The match options apply to the files or directories matched by the rule. The match options are:

**`EXCLUDE`**

Exclude the matched file or directory from installation.

**`PERMISSIONS <permission>...`**

Override the permissions setting for the matched file or directory.

For example, the code

~~~
install(DIRECTORY icons scripts/ DESTINATION share/myproj
        PATTERN "CVS" EXCLUDE
        PATTERN "scripts/*"
        PERMISSIONS OWNER_EXECUTE OWNER_WRITE OWNER_READ
                    GROUP_EXECUTE GROUP_READ)
~~~

will install the `icons` directory to `share/myproj/icons` and the `scripts` directory to `share/myproj`. The icons will get default file permissions, the scripts will be given specific permissions, and any `CVS` directories will be excluded.

**install(SCRIPT \<file\> [...])**

**install(CODE \<code\> [...])**

Invoke CMake scripts or code during installation:

~~~
install([[SCRIPT <file>] [CODE <code>]]
        [ALL_COMPONENTS | COMPONENT <component>]
        [EXCLUDE_FROM_ALL])
~~~

The `SCRIPT` form will invoke the given CMake script files during installation. If the script file name is a relative path it will be interpreted with respect to the current source directory. The `CODE` form will invoke the given CMake code during installation. Code is specified as a single argument inside a double-quoted string. For example, the code

~~~
install(CODE "message(\"Sample install message.\")")
~~~

will print a message during installation.

Added in version 3.21: When the `ALL_COMPONENTS` option is given, the custom installation script code will be executed for every component of a component-specific installation. This option is mutually exclusive with the `COMPONENT` option.

Added in version 3.14: `<file>` or `<code>` may use "generator expressions" with the syntax `$<...>` (in the case of `<file>`, this refers to their use in the file name, not the file's contents). See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions.

**install(EXPORT \<export-name\> [...])**

Install a CMake file exporting targets for dependent projects:

~~~
install(EXPORT <export-name> DESTINATION <dir>
        [NAMESPACE <namespace>] [FILE <name>.cmake]
        [PERMISSIONS <permission>...]
        [CONFIGURATIONS <config>...]
        [CXX_MODULES_DIRECTORY <directory>]
        [EXPORT_LINK_INTERFACE_LIBRARIES]
        [COMPONENT <component>]
        [EXCLUDE_FROM_ALL]
        [EXPORT_PACKAGE_DEPENDENCIES])
install(EXPORT_ANDROID_MK <export-name> DESTINATION <dir> [...])
~~~

The `EXPORT` form generates and installs a CMake file containing code to import targets from the installation tree into another project. Target installations are associated with the export `<export-name>` using the `EXPORT` option of the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) signature documented above. The `NAMESPACE` option will prepend `<namespace>` to the target names as they are written to the import file. By default the generated file will be called `<export-name>.cmake` but the `FILE` option may be used to specify a different name. The value given to the `FILE` option must be a file name with the `.cmake` extension.

If a `CONFIGURATIONS` option is given then the file will only be installed when one of the named configurations is installed. Additionally, the generated import file will reference only the matching target configurations. See the [`CMAKE_MAP_IMPORTED_CONFIG_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MAP_IMPORTED_CONFIG_CONFIG.html#variable:CMAKE_MAP_IMPORTED_CONFIG_%3CCONFIG%3E) variable to map configurations of dependent projects to the installed configurations. The `EXPORT_LINK_INTERFACE_LIBRARIES` keyword, if present, causes the contents of the properties matching `(IMPORTED_)?LINK_INTERFACE_LIBRARIES(_<CONFIG>)?` to be exported, when policy [`CMP0022`](https://cmake.org/cmake/help/v4.4/policy/CMP0022.html#policy:CMP0022) is `NEW`.

Note

The installed `<export-name>.cmake` file may come with additional per-configuration `<export-name>-*.cmake` files to be loaded by globbing. Do not use an export name that is the same as the package name in combination with installing a `<package-name>-config.cmake` file or the latter may be incorrectly matched by the glob and loaded.

When a `COMPONENT` option is given, the listed `<component>` implicitly depends on all components mentioned in the export set. The exported `<name>.cmake` file will require each of the exported components to be present in order for dependent projects to build properly. For example, a project may define components `Runtime` and `Development`, with shared libraries going into the `Runtime` component and static libraries and headers going into the `Development` component. The export set would also typically be part of the `Development` component, but it would export targets from both the `Runtime` and `Development` components. Therefore, the `Runtime` component would need to be installed if the `Development` component was installed, but not vice versa. If the `Development` component was installed without the `Runtime` component, dependent projects that try to link against it would have build errors. Package managers, such as APT and RPM, typically handle this by listing the `Runtime` component as a dependency of the `Development` component in the package metadata, ensuring that the library is always installed if the headers and CMake export file are present.

Added in version 3.7: In addition to cmake language files, the `EXPORT_ANDROID_MK` mode may be used to specify an export to the android ndk build system. This mode accepts the same options as the normal export mode. The Android NDK supports the use of prebuilt libraries, both static and shared. This allows cmake to build the libraries of a project and make them available to an ndk build system complete with transitive dependencies, include flags and defines required to use the libraries.

**`CXX_MODULES_DIRECTORY`**

Added in version 3.28.

Specify a subdirectory to store C++ module information for targets in the export set. This directory will be populated with files which add the necessary target property information to the relevant targets. Note that without this information, none of the C++ modules which are part of the targets in the export set will support being imported in consuming targets.

**`EXPORT_PACKAGE_DEPENDENCIES`**

Note

Experimental. Gated by `CMAKE_EXPERIMENTAL_EXPORT_PACKAGE_DEPENDENCIES`.

Specify that [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) calls should be exported. If this argument is specified, CMake examines all targets in the export set and gathers their `INTERFACE` link targets. If any such targets either were found with [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) or have the [`EXPORT_FIND_PACKAGE_NAME`](https://cmake.org/cmake/help/v4.4/prop_tgt/EXPORT_FIND_PACKAGE_NAME.html#prop_tgt:EXPORT_FIND_PACKAGE_NAME) property set, and such package dependency was not disabled by passing `ENABLED OFF` to [`export(SETUP)`](https://cmake.org/cmake/help/v4.4/command/export.html#setup), then a [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) call is written with the target's corresponding package name, a `REQUIRED` argument, and any additional arguments specified by the `EXTRA_ARGS` argument of [`export(SETUP)`](https://cmake.org/cmake/help/v4.4/command/export.html#setup). Any package dependencies that were manually specified by passing `ENABLED ON` to [`export(SETUP)`](https://cmake.org/cmake/help/v4.4/command/export.html#setup) are also added, even if the exported targets don't depend on any targets from them.

The [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) calls are written in the following order:

- Any package dependencies that were listed in [`export(SETUP)`](https://cmake.org/cmake/help/v4.4/command/export.html#setup) are written in the order they were first specified, regardless of whether or not they contain `INTERFACE` dependencies of the exported targets.

- Any package dependencies that contain `INTERFACE` link dependencies of the exported targets and that were never specified in [`export(SETUP)`](https://cmake.org/cmake/help/v4.4/command/export.html#setup) are written in the order they were first found.

The `EXPORT` form is useful to help outside projects use targets built and installed by the current project. For example, the code

~~~
install(TARGETS myexe EXPORT myproj DESTINATION bin)
install(EXPORT myproj NAMESPACE mp_ DESTINATION lib/myproj)
install(EXPORT_ANDROID_MK myproj DESTINATION share/ndk-modules)
~~~

will install the executable `myexe` to `<prefix>/bin` and code to import it in the file `<prefix>/lib/myproj/myproj.cmake` and `<prefix>/share/ndk-modules/Android.mk`. An outside project may load this file with the include command and reference the `myexe` executable from the installation tree using the imported target name `mp_myexe` as if the target were built in its own tree.

**install(PACKAGE_INFO \<package-name\> [...])**

Added in version 4.3.

Installs a [Common Package Specification](https://cps-org.github.io/cps/) ("CPS") file exporting targets for dependent projects:

~~~
install(PACKAGE_INFO <package-name> EXPORT <export-name>
        [PROJECT <project-name>|NO_PROJECT_METADATA]
        [APPENDIX <appendix-name>]
        [DESTINATION <dir>]
        [LOWER_CASE_FILE]
        [VERSION <version>
         [COMPAT_VERSION <version>]
         [VERSION_SCHEMA <string>]]
        [DEFAULT_TARGETS <target>...]
        [DEFAULT_CONFIGURATIONS <config>...]
        [LICENSE <license-string>]
        [DEFAULT_LICENSE <license-string>]
        [DESCRIPTION <description-string>]
        [HOMEPAGE_URL <url-string>]
        [PERMISSIONS <permission>...]
        [CONFIGURATIONS <config>...]
        [CXX_MODULES_DIRECTORY <directory>]
        [COMPONENT <component>]
        [EXCLUDE_FROM_ALL])
~~~

The `PACKAGE_INFO` form generates and installs a Common Package Specification file which describes installed targets such that they can be consumed by another project. Target installations are associated with the export `<export-name>` using the `EXPORT` option of the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) signature documented above. Unlike [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export), this information is not expressed in CMake code, and can be consumed by tools other than CMake. When imported into another CMake project, the imported targets will be prefixed with `<package-name>::`. By default, the generated file will be called `<package-name>[-<appendix-name>].cps`. If `LOWER_CASE_FILE` is given, the package name as it appears on disk (in both the file name and install destination) will be first converted to lower case.

If `DESTINATION` is not specified, a platform-specific default is used.

Several options may be used to specify package metadata:

**`VERSION <version>`**

Version of the package. The `<version>` shall conform to the specified schema. Refer to [Version Selection (CPS)](https://cmake.org/cmake/help/v4.4/command/find_package.html#cps-version-selection) for more information on how the package version is used when consumers request a package.

**`COMPAT_VERSION <version>`**

Oldest version for which the package provides compatibility.

If not specified, `COMPAT_VERSION` is implicitly taken to equal the package's `VERSION`, which is to say that no backwards compatibility is provided.

**`VERSION_SCHEMA <schema>`**

The schema that the package's version number(s) (both `VERSION` and `COMPAT_VERSION`) follow. While no schema will be written to the `.cps` file if this option is not provided, CPS specifies that the schema is assumed to be `simple` in such cases. Refer to [`version_schema`](https://cps-org.github.io/cps/schema.html#version-schema) for more details and a list of officially supported schemas, but be aware that the specification may include schemas that are not supported by CMake. See [Version Selection (CPS)](https://cmake.org/cmake/help/v4.4/command/find_package.html#cps-version-selection) for the list of schemas supported by [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package).

`DEFAULT_TARGETS <target>...`

Targets to be used if a consumer requests linking to the package name, rather than to specific components.

`DEFAULT_CONFIGURATIONS <config>...`

Ordered list of configurations consumers should prefer if no exact match or mapping of the consumer's configuration to the package's available configurations exists. If not specified, CMake will fall back to the package's available configurations in an unspecified order.

**`LICENSE <license-string>`**

Added in version 4.2.

A [System Package Data Exchange](https://spdx.dev/) (SPDX) [License Expression](https://spdx.github.io/spdx-spec/v3.0.1/annexes/spdx-license-expressions/) that describes the license(s) of the project as a whole, including documentation, resources, or other materials distributed with the project, in addition to software artifacts. See the SPDX [License List](https://spdx.org/licenses/) for a list of commonly used licenses and their identifiers.

The license of individual components is taken from the [`SPDX_LICENSE`](https://cmake.org/cmake/help/v4.4/prop_tgt/SPDX_LICENSE.html#prop_tgt:SPDX_LICENSE) property of their respective targets.

**`DEFAULT_LICENSE <license-string>`**

Added in version 4.2.

A [System Package Data Exchange](https://spdx.dev/) (SPDX) [License Expression](https://spdx.github.io/spdx-spec/v3.0.1/annexes/spdx-license-expressions/) that describes the license(s) of any components which do not otherwise specify their license(s).

**`DESCRIPTION <description-string>`**

Added in version 4.1.

An informational description of the project. It is recommended that this description is a relatively short string, usually no more than a few words.

**`HOMEPAGE_URL <url-string>`**

Added in version 4.1.

An informational canonical home URL for the project.

By default, if the specified `<package-name>` matches the current CMake [`PROJECT_NAME`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_NAME.html#variable:PROJECT_NAME), package metadata will be inherited from the project. The `PROJECT <project-name>` option may be used to specify a different project from which to inherit metadata. If `NO_PROJECT_METADATA` is specified, automatic inheritance of package metadata will be disabled. In any case, any metadata values specified in the `install` command will take precedence.

If `APPENDIX` is specified, rather than generating a top level package specification, the specified targets will be exported as an appendix to the named package. Appendices may be used to separate less commonly used targets (along with their external dependencies) from the rest of a package. This enables consumers to ignore transitive dependencies for targets that they don't use, and also allows a single logical "package" to be composed of artifacts produced by multiple build trees.

Appendices are not permitted to change basic package metadata; therefore, none of `PROJECT`, `VERSION`, `COMPAT_VERSION`, `VERSION_SCHEMA`, `DEFAULT_TARGETS` or `DEFAULT_CONFIGURATIONS` may be specified in combination with `APPENDIX`. Additionally, it is strongly recommended that use of `LOWER_CASE_FILE` should be consistent between the main package and any appendices.

Note

Because it is intended to be portable across multiple build tools, CPS may not support all features that are allowed in CMake-script exports. In particular, support for generator expressions in interface properties is limited at this time to configuration-dependent expressions.

Note

This is the recommended way to generate Common Package Specification package information for a project. For distributors whose users may require CPS package information when making changes to the project's build files is not practical, the [`CMAKE_INSTALL_EXPORTS_AS_PACKAGE_INFO`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_EXPORTS_AS_PACKAGE_INFO.html#variable:CMAKE_INSTALL_EXPORTS_AS_PACKAGE_INFO) variable may be used to generate `.cps` files from [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) calls. Refer to the variable's documentation for usage and caveats.

**install(RUNTIME_DEPENDENCY_SET \<set-name\> [...])**

Added in version 3.21.

Installs a runtime dependency set:

~~~
install(RUNTIME_DEPENDENCY_SET <set-name>
        [[LIBRARY|RUNTIME|FRAMEWORK]
         [DESTINATION <dir>]
         [PERMISSIONS <permission>...]
         [CONFIGURATIONS <config>...]
         [COMPONENT <component>]
         [NAMELINK_COMPONENT <component>]
         [OPTIONAL] [EXCLUDE_FROM_ALL]
        ]...
        [PRE_INCLUDE_REGEXES <regex>...]
        [PRE_EXCLUDE_REGEXES <regex>...]
        [POST_INCLUDE_REGEXES <regex>...]
        [POST_EXCLUDE_REGEXES <regex>...]
        [POST_INCLUDE_FILES <file>...]
        [POST_EXCLUDE_FILES <file>...]
        [DIRECTORIES <dir>...]
        )
~~~

Installs a runtime dependency set previously created by one or more [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) or [`install(IMPORTED_RUNTIME_ARTIFACTS)`](https://cmake.org/cmake/help/v4.4/command/install.html#imported-runtime-artifacts) commands. The dependencies of targets belonging to a runtime dependency set are installed in the `RUNTIME` destination and component on DLL platforms, and in the `LIBRARY` destination and component on non-DLL platforms. macOS frameworks are installed in the `FRAMEWORK` destination and component. Targets built within the build tree will never be installed as runtime dependencies, nor will their own dependencies, unless the targets themselves are installed with [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets).

The generated install script calls [`file(GET_RUNTIME_DEPENDENCIES)`](https://cmake.org/cmake/help/v4.4/command/file.html#get-runtime-dependencies) on the build-tree files to calculate the runtime dependencies. The build-tree executable files are passed as the `EXECUTABLES` argument, the build-tree shared libraries as the `LIBRARIES` argument, and the build-tree modules as the `MODULES` argument. On macOS, if one of the executables is a [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE), that executable is passed as the `BUNDLE_EXECUTABLE` argument. At most one such bundle executable may be in the runtime dependency set on macOS. The [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) property has no effect on other platforms. Note that [`file(GET_RUNTIME_DEPENDENCIES)`](https://cmake.org/cmake/help/v4.4/command/file.html#get-runtime-dependencies) only supports collecting the runtime dependencies for Windows, Linux and macOS platforms, so `install(RUNTIME_DEPENDENCY_SET)` has the same limitation.

The following sub-arguments are forwarded through as the corresponding arguments to [`file(GET_RUNTIME_DEPENDENCIES)`](https://cmake.org/cmake/help/v4.4/command/file.html#get-runtime-dependencies) (for those that provide a non-empty list of directories, regular expressions or files). They all support [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

- `DIRECTORIES <dir>...`

- `PRE_INCLUDE_REGEXES <regex>...`

- `PRE_EXCLUDE_REGEXES <regex>...`

- `POST_INCLUDE_REGEXES <regex>...`

- `POST_EXCLUDE_REGEXES <regex>...`

- `POST_INCLUDE_FILES <file>...`

- `POST_EXCLUDE_FILES <file>...`

Note

This command supersedes the [`install_targets()`](https://cmake.org/cmake/help/v4.4/command/install_targets.html#command:install_targets) command and the [`PRE_INSTALL_SCRIPT`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRE_INSTALL_SCRIPT.html#prop_tgt:PRE_INSTALL_SCRIPT) and [`POST_INSTALL_SCRIPT`](https://cmake.org/cmake/help/v4.4/prop_tgt/POST_INSTALL_SCRIPT.html#prop_tgt:POST_INSTALL_SCRIPT) target properties. It also replaces the `FILES` forms of the [`install_files()`](https://cmake.org/cmake/help/v4.4/command/install_files.html#command:install_files) and [`install_programs()`](https://cmake.org/cmake/help/v4.4/command/install_programs.html#command:install_programs) commands. The processing order of these install rules relative to those generated by [`install_targets()`](https://cmake.org/cmake/help/v4.4/command/install_targets.html#command:install_targets), [`install_files()`](https://cmake.org/cmake/help/v4.4/command/install_files.html#command:install_files), and [`install_programs()`](https://cmake.org/cmake/help/v4.4/command/install_programs.html#command:install_programs) commands is not defined.

**install(SBOM \<sbom-name\> [...])**

Added in version 4.3.

Note

Experimental. Gated by `CMAKE_EXPERIMENTAL_GENERATE_SBOM`.

Installs a Software Bill of Material or "SBOM" which describes the project:

~~~
install(SBOM <sbom-name> EXPORTS <export-names>...
        [PROJECT <project-name>|NO_PROJECT_METADATA]
        [DESTINATION <dir>]
        [VERSION <major>[.<minor>[.<patch>[.<tweak>]]]]
        [LICENSE <license-string>]
        [DESCRIPTION <description-string>]
        [HOMEPAGE_URL <url-string>]
        [PACKAGE_URL <url-string>]
        [FORMAT <string>])
~~~

The `SBOM` form generates a Software Bill of Material or "SBOM" file for a given project and installs it as part of the project installation. A Software Bill of Material is a machine-readable description of the project's targets, linked libraries, and related metadata, such as versions and license information. CMake currently generates SBOM files using the [System Package Data Exchange](https://spdx.dev/) 3.0 specification in its JSON-LD representation, as selected by the `FORMAT` option, but the interface is designed to allow additional SBOM formats or schema versions to be supported in future CMake releases.

Target installations are associated with each export `<export-names>` in the `EXPORTS` list using the `EXPORTS` option of the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) signature documented above. A single SBOM may cover multiple export sets; targets from all listed exports are aggregated into one document. If `DESTINATION` is not specified, a platform-specific default is used.

Several options may be used to specify package metadata:

**`VERSION <version>`**

The package version, specified as a series of non-negative integer components, i.e. \<major\>[.\<minor\>[.\<patch\>[.\<tweak\>]]]. See [`project(VERSION)`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) for more information.

**`FORMAT <string>`**

The format in which the SBOM should be exported, which must be an expression of the form `<format>[-<version>][+<representation>]`. CMake currently supports the JSON-LD serialization of [System Package Data Exchange](https://spdx.dev/) v3.0.1 (`spdx` or `spdx-3.0.1+json`), which is also the default if `FORMAT` is not specified.

`HOMEPAGE_URL <url-string>`

An informational canonical home URL for the project.

`PACKAGE_URL <url-string>`

An informational canonical package URL for the project.

`LICENSE <license-string>`

A [System Package Data Exchange](https://spdx.dev/) (SPDX) [License Expression](https://spdx.github.io/spdx-spec/v3.0.1/annexes/spdx-license-expressions/) that describes the license(s) of the project as a whole, including documentation, resources, or other materials distributed with the project, in addition to software artifacts. See the SPDX [License List](https://spdx.org/licenses/) for a list of commonly used licenses and their identifiers.

The license of individual components is taken from the [`SPDX_LICENSE`](https://cmake.org/cmake/help/v4.4/prop_tgt/SPDX_LICENSE.html#prop_tgt:SPDX_LICENSE) property of their respective targets.

`DESCRIPTION <description-string>`

An informational description of the project. It is recommended that this description is a relatively short string, usually no more than a few words.

By default, if the specified `<sbom-name>` matches the current CMake [`PROJECT_NAME`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_NAME.html#variable:PROJECT_NAME), sbom metadata will be inherited from the project. The `PROJECT <project-name>` option may be used to specify a different project from which to inherit metadata. If `NO_PROJECT_METADATA` is specified, automatic inheritance of sbom metadata will be disabled. In any case, any metadata values specified in the `install` command will take precedence.

Note that an SBOM file cannot be generated for targets with generator expressions contained in their [`LINK_LIBRARIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_LIBRARIES.html#prop_tgt:LINK_LIBRARIES) or [`INTERFACE_LINK_LIBRARIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_LINK_LIBRARIES.html#prop_tgt:INTERFACE_LINK_LIBRARIES) properties, unless the generator expressions are guarded by [`LINK_ONLY`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:LINK_ONLY).

## [Examples](https://cmake.org/cmake/help/v4.4/command/install.html#id6)

### [Example: Install Targets with Per-Artifact Components](https://cmake.org/cmake/help/v4.4/command/install.html#id7)

Consider a project that defines targets with different artifact kinds:

~~~
add_executable(myExe myExe.c)
add_library(myStaticLib STATIC myStaticLib.c)
target_sources(myStaticLib PUBLIC FILE_SET HEADERS FILES myStaticLib.h)
add_library(mySharedLib SHARED mySharedLib.c)
target_sources(mySharedLib PUBLIC FILE_SET HEADERS FILES mySharedLib.h)
set_property(TARGET mySharedLib PROPERTY SOVERSION 1)
~~~

We may call [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) with [\<artifact-kind\>](https://cmake.org/cmake/help/v4.4/command/install.html#artifact-kind) arguments to specify different options for each kind of artifact:

~~~
install(TARGETS
          myExe
          mySharedLib
          myStaticLib
        RUNTIME           # Following options apply to runtime artifacts.
          COMPONENT Runtime
        LIBRARY           # Following options apply to library artifacts.
          COMPONENT Runtime
          NAMELINK_COMPONENT Development
        ARCHIVE           # Following options apply to archive artifacts.
          COMPONENT Development
          DESTINATION lib/static
        FILE_SET HEADERS  # Following options apply to file set HEADERS.
          COMPONENT Development
        )
~~~

This will:

- Install `myExe` to `<prefix>/bin`, the default RUNTIME artifact destination, as part of the `Runtime` component.

- On non-DLL platforms:

   

  - Install `libmySharedLib.so.1` to `<prefix>/lib`, the default LIBRARY artifact destination, as part of the `Runtime` component.

  - Install the `libmySharedLib.so` "namelink" (symbolic link) to `<prefix>/lib`, the default LIBRARY artifact destination, as part of the `Development` component.

- On DLL platforms:

   

  - Install `mySharedLib.dll` to `<prefix>/bin`, the default RUNTIME artifact destination, as part of the `Runtime` component.

  - Install `mySharedLib.lib` to `<prefix>/lib/static`, the specified ARCHIVE artifact destination, as part of the `Development` component.

- Install `myStaticLib` to `<prefix>/lib/static`, the specified ARCHIVE artifact destination, as part of the `Development` component.

- Install `mySharedLib.h` and `myStaticLib.h` to `<prefix>/include`, the default destination for a file set of type HEADERS, as part of the `Development` component.

### [Example: Install Targets to Per-Config Destinations](https://cmake.org/cmake/help/v4.4/command/install.html#id8)

Each [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) call installs a given target [output artifact](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#output-artifacts) to at most one `DESTINATION`, but the install rule itself may be filtered by the `CONFIGURATIONS` option. In order to install to a different destination for each configuration, one call per configuration is needed. For example, the code:

~~~
install(TARGETS myExe
        CONFIGURATIONS Debug
        RUNTIME
          DESTINATION Debug/bin
        )
install(TARGETS myExe
        CONFIGURATIONS Release
        RUNTIME
          DESTINATION Release/bin
        )
~~~

will install `myExe` to `<prefix>/Debug/bin` in the Debug configuration, and to `<prefix>/Release/bin` in the Release configuration.

## [Generated Installation Script](https://cmake.org/cmake/help/v4.4/command/install.html#id9)

Note

Use of this feature is not recommended. Please consider using the [`cmake --install`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-install) instead.

The `install()` command generates a file, `cmake_install.cmake`, inside the build directory, which is used internally by the generated install target and by CPack. You can also invoke this script manually with [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P). This script accepts several variables:

**`COMPONENT`**

Set this variable to install only a single CPack component as opposed to all of them. For example, if you only want to install the `Development` component, run `cmake -DCOMPONENT=Development -P cmake_install.cmake`.

**`BUILD_TYPE`**

Set this variable to change the build type if you are using a multi-config generator. For example, to install with the `Debug` configuration, run `cmake -DBUILD_TYPE=Debug -P cmake_install.cmake`.

**`DESTDIR`**

This is an environment variable rather than a CMake variable. It allows you to change the installation prefix on UNIX systems. See [`DESTDIR`](https://cmake.org/cmake/help/v4.4/envvar/DESTDIR.html#envvar:DESTDIR) for details.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
