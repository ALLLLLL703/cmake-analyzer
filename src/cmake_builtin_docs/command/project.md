CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/project.html)

# project

Set the name of the project.

## Synopsis

~~~
project(<PROJECT-NAME> [<language-name>...])
project(<PROJECT-NAME>
        [VERSION <major>[.<minor>[.<patch>[.<tweak>]]]]
        [COMPAT_VERSION <major>[.<minor>[.<patch>[.<tweak>]]]]
        [SPDX_LICENSE <license-string>]
        [DESCRIPTION <description-string>]
        [HOMEPAGE_URL <url-string>]
        [LANGUAGES <language-name>...])
~~~

Sets the name of the project, and stores it in the variable [`PROJECT_NAME`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_NAME.html#variable:PROJECT_NAME). When called from the top-level `CMakeLists.txt` also stores the project name in the variable [`CMAKE_PROJECT_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_NAME.html#variable:CMAKE_PROJECT_NAME).

Also sets the variables:

**[`PROJECT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_SOURCE_DIR.html#variable:PROJECT_SOURCE_DIR), [`<PROJECT-NAME>_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_SOURCE_DIR.html#variable:%3CPROJECT-NAME%3E_SOURCE_DIR)**

Absolute path to the source directory for the project.

**[`PROJECT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_BINARY_DIR.html#variable:PROJECT_BINARY_DIR), [`<PROJECT-NAME>_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_BINARY_DIR.html#variable:%3CPROJECT-NAME%3E_BINARY_DIR)**

Absolute path to the binary directory for the project.

**[`PROJECT_IS_TOP_LEVEL`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_IS_TOP_LEVEL.html#variable:PROJECT_IS_TOP_LEVEL), [`<PROJECT-NAME>_IS_TOP_LEVEL`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_IS_TOP_LEVEL.html#variable:%3CPROJECT-NAME%3E_IS_TOP_LEVEL)**

Added in version 3.21.

Boolean value indicating whether the project is top-level.

Further variables are set by the optional arguments described in [Options](https://cmake.org/cmake/help/v4.4/command/project.html#options) further below. Where an option is not given, its corresponding variable is set to the empty string.

Note that variables of the form `<name>_SOURCE_DIR` and `<name>_BINARY_DIR` may also be set by other commands before `project()` is called (see the [`FetchContent_MakeAvailable()`](https://cmake.org/cmake/help/v4.4/module/FetchContent.html#command:fetchcontent_makeavailable) command for one example). Projects should not rely on `<PROJECT-NAME>_SOURCE_DIR` or `<PROJECT-NAME>_BINARY_DIR` holding a particular value outside of the scope of the call to `project()` or one of its child scopes.

Changed in version 3.30: `<PROJECT-NAME>_SOURCE_DIR`, `<PROJECT-NAME>_BINARY_DIR`, and `<PROJECT-NAME>_IS_TOP_LEVEL`, if already set as normal variables when `project(<PROJECT-NAME> ...)` is called, are updated by the call. Cache entries by the same names are always set as before. See release notes for 3.30.3, 3.30.4, and 3.30.5 for details.

Changed in version 3.31: `<PROJECT-NAME>_SOURCE_DIR`, `<PROJECT-NAME>_BINARY_DIR`, and `<PROJECT-NAME>_IS_TOP_LEVEL` are always set as normal variables by `project(<PROJECT-NAME> ...)`. See policy [`CMP0180`](https://cmake.org/cmake/help/v4.4/policy/CMP0180.html#policy:CMP0180). Cache entries by the same names are always set as before.

## Options

The options are:

**`VERSION <version>`**

Optional; may not be used unless policy [`CMP0048`](https://cmake.org/cmake/help/v4.4/policy/CMP0048.html#policy:CMP0048) is set to `NEW`.

Takes a `<version>` argument composed of non-negative integer components, i.e. `<major>[.<minor>[.<patch>[.<tweak>]]]`, and sets the variables

- [`PROJECT_VERSION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_VERSION.html#variable:PROJECT_VERSION), [`<PROJECT-NAME>_VERSION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_VERSION.html#variable:%3CPROJECT-NAME%3E_VERSION)

- [`PROJECT_VERSION_MAJOR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_VERSION_MAJOR.html#variable:PROJECT_VERSION_MAJOR), [`<PROJECT-NAME>_VERSION_MAJOR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_VERSION_MAJOR.html#variable:%3CPROJECT-NAME%3E_VERSION_MAJOR)

- [`PROJECT_VERSION_MINOR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_VERSION_MINOR.html#variable:PROJECT_VERSION_MINOR), [`<PROJECT-NAME>_VERSION_MINOR`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_VERSION_MINOR.html#variable:%3CPROJECT-NAME%3E_VERSION_MINOR)

- [`PROJECT_VERSION_PATCH`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_VERSION_PATCH.html#variable:PROJECT_VERSION_PATCH), [`<PROJECT-NAME>_VERSION_PATCH`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_VERSION_PATCH.html#variable:%3CPROJECT-NAME%3E_VERSION_PATCH)

- [`PROJECT_VERSION_TWEAK`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_VERSION_TWEAK.html#variable:PROJECT_VERSION_TWEAK), [`<PROJECT-NAME>_VERSION_TWEAK`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_VERSION_TWEAK.html#variable:%3CPROJECT-NAME%3E_VERSION_TWEAK).

Added in version 3.12: When the `project()` command is called from the top-level `CMakeLists.txt`, then the version is also stored in the variable [`CMAKE_PROJECT_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_VERSION.html#variable:CMAKE_PROJECT_VERSION).

**`COMPAT_VERSION <version>`**

Added in version 4.3.

Optional; requires `VERSION` also be set.

Takes a `<version>` argument composed of non-negative integer components, i.e. `<major>[.<minor>[.<patch>[.<tweak>]]]`, and sets the variables

- [`PROJECT_COMPAT_VERSION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_COMPAT_VERSION.html#variable:PROJECT_COMPAT_VERSION), [`<PROJECT-NAME>_COMPAT_VERSION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_COMPAT_VERSION.html#variable:%3CPROJECT-NAME%3E_COMPAT_VERSION).

When the `project()` command is called from the top-level `CMakeLists.txt`, then the compatibility version is also stored in the variable [`CMAKE_PROJECT_COMPAT_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_COMPAT_VERSION.html#variable:CMAKE_PROJECT_COMPAT_VERSION).

**`SPDX_LICENSE <license-string>`**

Added in version 4.3.

Optional. Sets the variables

- [`PROJECT_SPDX_LICENSE`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_SPDX_LICENSE.html#variable:PROJECT_SPDX_LICENSE), [`<PROJECT-NAME>_SPDX_LICENSE`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_SPDX_LICENSE.html#variable:%3CPROJECT-NAME%3E_SPDX_LICENSE)

to `<license-string>`, which shall be a [System Package Data Exchange](https://spdx.dev/) (SPDX) [License Expression](https://spdx.github.io/spdx-spec/v3.0.1/annexes/spdx-license-expressions/) that describes the license(s) of the project as a whole, including documentation, resources, or other materials distributed with the project, in addition to software artifacts. See the SPDX [License List](https://spdx.org/licenses/) for a list of commonly used licenses and their identifiers. See the [`SPDX_LICENSE`](https://cmake.org/cmake/help/v4.4/prop_tgt/SPDX_LICENSE.html#prop_tgt:SPDX_LICENSE) property for specifying the license(s) on individual software artifacts.

Note

The project license is *not* used to initialize the [`SPDX_LICENSE`](https://cmake.org/cmake/help/v4.4/prop_tgt/SPDX_LICENSE.html#prop_tgt:SPDX_LICENSE) property of individual targets. This allows the package license and default component license, which are specified when exporting package information, to be meaningful. Only Common Package Specification exports make use of this information.

The project license *is* inherited as the package license in some cases. Refer to the `PROJECT` option and related documentation of the [`export()`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) and [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) commands for more information.

**`DESCRIPTION <description-string>`**

Added in version 3.9.

Optional. Sets the variables

- [`PROJECT_DESCRIPTION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_DESCRIPTION.html#variable:PROJECT_DESCRIPTION), [`<PROJECT-NAME>_DESCRIPTION`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_DESCRIPTION.html#variable:%3CPROJECT-NAME%3E_DESCRIPTION)

to `<description-string>`. It is recommended that this description is a relatively short string, usually no more than a few words.

When the `project()` command is called from the top-level `CMakeLists.txt`, then the description is also stored in the variable [`CMAKE_PROJECT_DESCRIPTION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_DESCRIPTION.html#variable:CMAKE_PROJECT_DESCRIPTION).

Added in version 3.12: Added the `<PROJECT-NAME>_DESCRIPTION` variable.

**`HOMEPAGE_URL <url-string>`**

Added in version 3.12.

Optional. Sets the variables

- [`PROJECT_HOMEPAGE_URL`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_HOMEPAGE_URL.html#variable:PROJECT_HOMEPAGE_URL), [`<PROJECT-NAME>_HOMEPAGE_URL`](https://cmake.org/cmake/help/v4.4/variable/PROJECT-NAME_HOMEPAGE_URL.html#variable:%3CPROJECT-NAME%3E_HOMEPAGE_URL)

to `<url-string>`, which should be the canonical home URL for the project.

When the `project()` command is called from the top-level `CMakeLists.txt`, then the URL also is stored in the variable [`CMAKE_PROJECT_HOMEPAGE_URL`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_HOMEPAGE_URL.html#variable:CMAKE_PROJECT_HOMEPAGE_URL).

**`LANGUAGES <language-name>...`**

Optional. Can also be specified without `LANGUAGES` keyword per the first, short signature.

Selects which programming languages are needed to build the project.

The supported languages are:

`C`

**`CXX`**

C++

**`CSharp`**

Added in version 3.8.

C#

**`CUDA`**

Added in version 3.8.

**`OBJC`**

Added in version 3.16.

Objective-C

**`OBJCXX`**

Added in version 3.16.

Objective-C++

`Fortran`

**`HIP`**

Added in version 3.21.

**`ISPC`**

Added in version 3.18.

**`Swift`**

Added in version 3.15.

**`ASM`**

Assembly language supported by the C compiler.

If enabling `ASM`, list it last so that CMake can check whether the `C` or `CXX` compiler supports assembly.

**`ASM_NASM`**

Netwide Assembler

**`ASM_MARMASM`**

Added in version 3.26.

Microsoft Assembler (ARM, ARM64)

**`ASM_MASM`**

Microsoft Assembler (x86, x64)

**`ASM_POASM`**

Added in version 4.4.

Pelles C toolchain assembler.

`ASM-ATT`

By default `C` and `CXX` are enabled if no language options are given. Specify language `NONE`, or use the `LANGUAGES` keyword and list no languages, to skip enabling any languages.

The variables set through the `VERSION`, `COMPAT_VERSION`, `SPDX_LICENSE`, `DESCRIPTION` and `HOMEPAGE_URL` options are intended for use as default values in package metadata and documentation. The [`export()`](https://cmake.org/cmake/help/v4.4/command/export.html#command:export) and [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) commands use these accordingly when generating Common Package Specification package descriptions.

## Code Injection

A number of variables can be defined by the user to specify files to include at different points during the execution of the `project()` command. The following outlines the steps performed during a `project()` call:

- Added in version 3.15: For every `project()` call regardless of the project name, include the file(s) and module(s) named by [`CMAKE_PROJECT_INCLUDE_BEFORE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_INCLUDE_BEFORE.html#variable:CMAKE_PROJECT_INCLUDE_BEFORE), if set.

- Added in version 3.17: If the `project()` command specifies `<PROJECT-NAME>` as its project name, include the file(s) and module(s) named by [`CMAKE_PROJECT_<PROJECT-NAME>_INCLUDE_BEFORE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_PROJECT-NAME_INCLUDE_BEFORE.html#variable:CMAKE_PROJECT_%3CPROJECT-NAME%3E_INCLUDE_BEFORE), if set.

- Set the various project-specific variables detailed in the [Synopsis](https://cmake.org/cmake/help/v4.4/command/project.html#synopsis) and [Options](https://cmake.org/cmake/help/v4.4/command/project.html#options) sections above.

- For the very first `project()` call only:

   

  - If [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) is set, read it at least once. It may be read multiple times and it may also be read again when enabling languages later (see below).

  - Set the variables describing the host and target platforms. Language-specific variables might or might not be set at this point. On the first run, the only language-specific variables that might be defined are those a toolchain file may have set. On subsequent runs, language-specific variables cached from a previous run may be set.

  - Added in version 3.24: Include each file listed in [`CMAKE_PROJECT_TOP_LEVEL_INCLUDES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_TOP_LEVEL_INCLUDES.html#variable:CMAKE_PROJECT_TOP_LEVEL_INCLUDES), if set. The variable is ignored by CMake thereafter.

- Enable any languages specified in the call, or the default languages if none were provided. The toolchain file may be re-read when enabling a language for the first time.

- Added in version 3.15: For every `project()` call regardless of the project name, include the file(s) and module(s) named by [`CMAKE_PROJECT_INCLUDE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_INCLUDE.html#variable:CMAKE_PROJECT_INCLUDE), if set.

- If the `project()` command specifies `<PROJECT-NAME>` as its project name, include the file(s) and module(s) named by [`CMAKE_PROJECT_<PROJECT-NAME>_INCLUDE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_PROJECT-NAME_INCLUDE.html#variable:CMAKE_PROJECT_%3CPROJECT-NAME%3E_INCLUDE), if set.

## Usage

The top-level `CMakeLists.txt` file for a project must contain a literal, direct call to the `project()` command; loading one through the [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include) command is not sufficient. If no such call exists, CMake will issue a warning and pretend there is a `project(Project)` at the top to enable the default languages (`C` and `CXX`).

Note

Call the `project()` command near the top of the top-level `CMakeLists.txt`, but *after* calling [`cmake_minimum_required()`](https://cmake.org/cmake/help/v4.4/command/cmake_minimum_required.html#command:cmake_minimum_required). It is important to establish version and policy settings before invoking other commands whose behavior they may affect and for this reason the `project()` command will issue a warning if this order is not kept. See also policy [`CMP0000`](https://cmake.org/cmake/help/v4.4/policy/CMP0000.html#policy:CMP0000).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
