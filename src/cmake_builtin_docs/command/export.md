CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/export.html)

# export

Export targets or packages for outside projects to use them directly from the current project's build tree, without installation.

See the [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) command to export targets from an install tree.

## Synopsis

~~~
export(TARGETS <target>... [...])
export(EXPORT <export-name> [...])
export(PACKAGE <PackageName>)
export(SETUP <export-name> [...])
export(PACKAGE_INFO <package-name> [...])
export(SBOM <sbom-name> [...])
~~~

## Exporting Targets

**export(TARGETS \<target\>... [...])**

~~~
export(TARGETS <target>... [NAMESPACE <namespace>]
       [APPEND] FILE <filename> [EXPORT_LINK_INTERFACE_LIBRARIES]
       [CXX_MODULES_DIRECTORY <directory>])
~~~

Creates a file `<filename>` that may be included by outside projects to import targets named by `<target>...` from the current project's build tree. This is useful during cross-compiling to build utility executables that can run on the host platform in one project and then import them into another project being compiled for the target platform.

The file created by this command is specific to the build tree and should never be installed. See the [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) command to export targets from an install tree.

The options are:

**`NAMESPACE <namespace>`**

Prepend the `<namespace>` string to all target names written to the file.

**`APPEND`**

Append to the file instead of overwriting it. This can be used to incrementally export multiple targets to the same file.

**`EXPORT_LINK_INTERFACE_LIBRARIES`**

Include the contents of the properties named with the pattern `(IMPORTED_)?LINK_INTERFACE_LIBRARIES(_<CONFIG>)?` in the export, even when policy [`CMP0022`](https://cmake.org/cmake/help/v4.4/policy/CMP0022.html#policy:CMP0022) is NEW. This is useful to support consumers using CMake versions older than 2.8.12.

**`CXX_MODULES_DIRECTORY <directory>`**

Added in version 3.28.

Export C++ module properties to files under the given directory. Each file will be named according to the target's export name (without any namespace). These files will automatically be included from the export file.

This signature requires all targets to be listed explicitly. If a library target is included in the export, but a target to which it links is not included, the behavior is unspecified. See the [`export(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) signature to automatically export the same targets from the build tree as [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) would from an install tree.

Note

[Object Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#object-libraries) under [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) have special handling if multiple architectures are listed in [`CMAKE_OSX_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_ARCHITECTURES.html#variable:CMAKE_OSX_ARCHITECTURES). In this case they will be exported as [Interface Libraries](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries) with no object files available to clients. This is sufficient to satisfy transitive usage requirements of other targets that link to the object libraries in their implementation.

This command exports all [Build Configurations](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#build-configurations) from the build tree. See the [`CMAKE_MAP_IMPORTED_CONFIG_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MAP_IMPORTED_CONFIG_CONFIG.html#variable:CMAKE_MAP_IMPORTED_CONFIG_%3CCONFIG%3E) variable to map configurations of dependent projects to the exported configurations.

### Exporting Targets to Android.mk

~~~
export(TARGETS <target>... ANDROID_MK <filename>)
~~~

Added in version 3.7.

This signature exports CMake built targets to the android ndk build system by creating an `Android.mk` file that references the prebuilt targets. The Android NDK supports the use of prebuilt libraries, both static and shared. This allows CMake to build the libraries of a project and make them available to an ndk build system complete with transitive dependencies, include flags and defines required to use the libraries. The signature takes a list of targets and puts them in the `Android.mk` file specified by the `<filename>` given. This signature can only be used if policy [`CMP0022`](https://cmake.org/cmake/help/v4.4/policy/CMP0022.html#policy:CMP0022) is NEW for all targets given. A error will be issued if that policy is set to OLD for one of the targets.

## Exporting Targets matching install(EXPORT)

**export(EXPORT \<export-name\> [...])**

~~~
export(EXPORT <export-name> [NAMESPACE <namespace>] [FILE <filename>]
       [CXX_MODULES_DIRECTORY <directory>] [EXPORT_PACKAGE_DEPENDENCIES])
~~~

Creates a file `<filename>` that may be included by outside projects to import targets from the current project's build tree. This is the same as the [`export(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/export.html#targets) signature, except that the targets are not explicitly listed. Instead, it exports the targets associated with the installation export `<export-name>`. Target installations may be associated with the export `<export-name>` using the `EXPORT` option of the [`install(TARGETS)`](https://cmake.org/cmake/help/v4.4/command/install.html#targets) command.

**`EXPORT_PACKAGE_DEPENDENCIES`**

Note

Experimental. Gated by `CMAKE_EXPERIMENTAL_EXPORT_PACKAGE_DEPENDENCIES`.

Specify that [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) calls should be exported. See [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export) for details on how this works.

### Exporting Targets to the Common Package Specification

**export(PACKAGE_INFO \<package-name\> [...])**

~~~
export(PACKAGE_INFO <package-name> EXPORT <export-name>
       [PROJECT <project-name>|NO_PROJECT_METADATA]
       [APPENDIX <appendix-name>]
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
       [CXX_MODULES_DIRECTORY <directory>])
~~~

Added in version 4.3.

Creates a file in the [Common Package Specification](https://cps-org.github.io/cps/) that may be included by outside projects to import targets named by `<target>...` from the current project's build tree. See the [`install(PACKAGE_INFO)`](https://cmake.org/cmake/help/v4.4/command/install.html#package-info) command to export targets from an install tree. The imported targets are implicitly in the namespace `<package-name>`.

The default file name is `<package-name>[-<appendix-name>].cps`. If the `LOWER_CASE_FILE` option is given, the file name will use the package name converted to lower case. Files are written to the `cps/<package-name>` subdirectory of the current build directory.

See [`install(PACKAGE_INFO)`](https://cmake.org/cmake/help/v4.4/command/install.html#package-info) for a description of the other options.

### Exporting Software Bill of Materials (SBOM) Documents

**export(SBOM \<sbom-name\> [...])**

~~~
export(SBOM <sbom-name> EXPORTS <export-names>...
       [FORMAT <string>]
       [PROJECT <project-name>|NO_PROJECT_METADATA]
       [VERSION <major>[.<minor>[.<patch>[.<tweak>]]]]
       [LICENSE <license-string>]
       [DESCRIPTION <description-string>]
       [HOMEPAGE_URL <url-string>]
       [PACKAGE_URL <url-string>])
~~~

Added in version 4.3.

Note

Experimental. Gated by `CMAKE_EXPERIMENTAL_GENERATE_SBOM`.

Generates a software bill of materials (SBOM) document describing the targets in the listed `<export-names>` export sets and their dependencies in the build tree. Targets from all listed exports are aggregated into one document. The generated SBOM will be written to the `sbom/<sbom-name>` subdirectory of the current build directory.

See [`install(SBOM)`](https://cmake.org/cmake/help/v4.4/command/install.html#sbom) for details about the supported SBOM formats and a description of the other options.

## Exporting Packages

**export(PACKAGE \<PackageName\>)**

~~~
export(PACKAGE <PackageName>)
~~~

Store the current build directory in the CMake user package registry for package `<PackageName>`. The [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) command may consider the directory while searching for package `<PackageName>`. This helps dependent projects find and use a package from the current project's build tree without help from the user. Note that the entry in the package registry that this command creates works only in conjunction with a package configuration file (`<PackageName>Config.cmake`) that works with the build tree. In some cases, for example for packaging and for system wide installations, it is not desirable to write the user package registry.

Changed in version 3.1: If the [`CMAKE_EXPORT_NO_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_NO_PACKAGE_REGISTRY.html#variable:CMAKE_EXPORT_NO_PACKAGE_REGISTRY) variable is enabled, the `export(PACKAGE)` command will do nothing.

Changed in version 3.15: By default the `export(PACKAGE)` command does nothing (see policy [`CMP0090`](https://cmake.org/cmake/help/v4.4/policy/CMP0090.html#policy:CMP0090)) because populating the user package registry has effects outside the source and build trees. Set the [`CMAKE_EXPORT_PACKAGE_REGISTRY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_PACKAGE_REGISTRY.html#variable:CMAKE_EXPORT_PACKAGE_REGISTRY) variable to add build directories to the CMake user package registry.

## Configuring Exports

**export(SETUP \<export-name\> [...])**

~~~
export(SETUP <export-name>
       [PACKAGE_DEPENDENCY <dep>
        [ENABLED (<bool-true>|<bool-false>|AUTO)]
        [EXTRA_ARGS <args>...]
       ] [...]
       [TARGET <target>
        [XCFRAMEWORK_LOCATION <location>]
       ] [...]
       )
~~~

Added in version 3.29.

Configure the parameters of an export. The arguments are as follows:

**`PACKAGE_DEPENDENCY <dep>`**

Note

Experimental. Gated by `CMAKE_EXPERIMENTAL_EXPORT_PACKAGE_DEPENDENCIES`.

Specify a package dependency to configure. This changes how [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) calls are written during [`export(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) and [`install(EXPORT)`](https://cmake.org/cmake/help/v4.4/command/install.html#export). `<dep>` is the name of a package to export. This argument accepts the following additional arguments:

**`ENABLED`**

Manually control whether or not the dependency is exported. This accepts the following values:

**`<bool-true>`**

Any value that CMake recognizes as "true". Always export the dependency, even if no exported targets depend on it. This can be used to manually add [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) calls to the export.

**`<bool-false>`**

Any value that CMake recognizes as "false". Never export the dependency, even if an exported target depends on it.

**`AUTO`**

Only export the dependency if an exported target depends on it.

**`EXTRA_ARGS <args>`**

Specify additional arguments to pass to [`find_dependency()`](https://cmake.org/cmake/help/v4.4/module/CMakeFindDependencyMacro.html#command:find_dependency) after the `REQUIRED` argument.

**`TARGET <target>`**

Specify a target to configure in this export. This argument accepts the following additional arguments:

**`XCFRAMEWORK_LOCATION`**

Specify the location of an `.xcframework` which contains the library from this target. If specified, the generated code will check to see if the `.xcframework` exists, and if it does, it will use the `.xcframework` as its imported location instead of the installed library.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
