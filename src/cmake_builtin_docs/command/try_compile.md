CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/try_compile.html)

# [try_compile](https://cmake.org/cmake/help/v4.4/command/try_compile.html#id3)

Contents

- [try_compile](https://cmake.org/cmake/help/v4.4/command/try_compile.html#try-compile)

   

  - [Try Compiling Whole Projects](https://cmake.org/cmake/help/v4.4/command/try_compile.html#try-compiling-whole-projects)

  - [Try Compiling Source Files](https://cmake.org/cmake/help/v4.4/command/try_compile.html#try-compiling-source-files)

  - [Options](https://cmake.org/cmake/help/v4.4/command/try_compile.html#options)

  - [Other Behavior Settings](https://cmake.org/cmake/help/v4.4/command/try_compile.html#other-behavior-settings)

  - [See Also](https://cmake.org/cmake/help/v4.4/command/try_compile.html#see-also)

Try building some code.

## [Try Compiling Whole Projects](https://cmake.org/cmake/help/v4.4/command/try_compile.html#id4)

~~~
try_compile(<compileResultVar> PROJECT <projectName>
            SOURCE_DIR <srcdir>
            [BINARY_DIR <bindir>]
            [TARGET <targetName>]
            [LOG_DESCRIPTION <text>]
            [NO_CACHE]
            [NO_LOG]
            [CMAKE_FLAGS <flags>...]
            [OUTPUT_VARIABLE <var>])
~~~

Added in version 3.25.

Try building a project. Build success returns `TRUE` and build failure returns `FALSE` in `<compileResultVar>`.

In this form, `<srcdir>` should contain a complete CMake project with a `CMakeLists.txt` file and all sources. The `<bindir>` and `<srcdir>` will not be deleted after this command is run. Specify `<targetName>` to build a specific target instead of the `all` or `ALL_BUILD` target. See below for the meaning of other options.

Changed in version 3.24: CMake variables describing platform settings, and those listed by the [`CMAKE_TRY_COMPILE_PLATFORM_VARIABLES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_PLATFORM_VARIABLES.html#variable:CMAKE_TRY_COMPILE_PLATFORM_VARIABLES) variable, are propagated into the project's build configuration. See policy [`CMP0137`](https://cmake.org/cmake/help/v4.4/policy/CMP0137.html#policy:CMP0137). Previously this was only done by the [source file](https://cmake.org/cmake/help/v4.4/command/try_compile.html#try-compiling-source-files) signature.

Added in version 3.26: This command records a [configure-log try_compile event](https://cmake.org/cmake/help/v4.4/manual/cmake-configure-log.7.html#try-compile-configure-log-event) if the `NO_LOG` option is not specified.

Added in version 3.30: If the [`PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE`](https://cmake.org/cmake/help/v4.4/prop_gbl/PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE.html#prop_gbl:PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE) global property is set to true, [`CMAKE_PROJECT_TOP_LEVEL_INCLUDES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_TOP_LEVEL_INCLUDES.html#variable:CMAKE_PROJECT_TOP_LEVEL_INCLUDES) is propagated into the project's build configuration.

This command supports an alternate signature for CMake older than 3.25. The signature above is recommended for clarity.

~~~
try_compile(<compileResultVar> <bindir> <srcdir>
            <projectName> [<targetName>]
            [CMAKE_FLAGS <flags>...]
            [OUTPUT_VARIABLE <var>])
~~~

## [Try Compiling Source Files](https://cmake.org/cmake/help/v4.4/command/try_compile.html#id5)

~~~
try_compile(<compileResultVar>
            [SOURCES_TYPE <type>]
            {SOURCES <srcfile>...                 |
             SOURCE_FROM_CONTENT <name> <content> |
             SOURCE_FROM_VAR <name> <var>         |
             SOURCE_FROM_FILE <name> <path>       }...
            [LOG_DESCRIPTION <text>]
            [NO_CACHE]
            [NO_LOG]
            [CMAKE_FLAGS <flags>...]
            [COMPILE_DEFINITIONS <defs>...]
            [LINK_OPTIONS <options>...]
            [LINK_LIBRARIES <libs>...]
            [LINKER_LANGUAGE <lang>]
            [OUTPUT_VARIABLE <var>]
            [COPY_FILE <fileName> [COPY_FILE_ERROR <var>]]
            [<LANG>_STANDARD <std>]
            [<LANG>_STANDARD_REQUIRED <bool>]
            [<LANG>_EXTENSIONS <bool>]
            )
~~~

Added in version 3.25.

Try building an executable or static library from one or more source files. The binary type is determined by variable [`CMAKE_TRY_COMPILE_TARGET_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_TARGET_TYPE.html#variable:CMAKE_TRY_COMPILE_TARGET_TYPE). Build success returns boolean `true` and build failure returns boolean `false` in `<compileResultVar>` (cached unless `NO_CACHE` is specified).

In this form, one or more source files must be provided. Additionally, one of `SOURCES` and/or `SOURCE_FROM_*` must precede other keywords.

If [`CMAKE_TRY_COMPILE_TARGET_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_TARGET_TYPE.html#variable:CMAKE_TRY_COMPILE_TARGET_TYPE) is unset or is set to `EXECUTABLE`, the sources must include a definition for `main` and CMake will create a `CMakeLists.txt` file to build the source(s) as an executable. If [`CMAKE_TRY_COMPILE_TARGET_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_TARGET_TYPE.html#variable:CMAKE_TRY_COMPILE_TARGET_TYPE) is set to `STATIC_LIBRARY`, a static library will be built instead and no definition for `main` is required. For an executable, the generated `CMakeLists.txt` file would contain something like the following:

~~~
add_definitions(<expanded COMPILE_DEFINITIONS from caller>)
include_directories(${INCLUDE_DIRECTORIES})
link_directories(${LINK_DIRECTORIES})
add_executable(cmTryCompileExec <srcfile>...)
target_link_options(cmTryCompileExec PRIVATE <LINK_OPTIONS from caller>)
target_link_libraries(cmTryCompileExec ${LINK_LIBRARIES})
~~~

CMake automatically generates, for each `try_compile` operation, a unique directory under `${CMAKE_BINARY_DIR}/CMakeFiles/CMakeScratch` with an unspecified name. These directories are cleaned automatically unless [`--debug-trycompile`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-debug-trycompile) is passed to **cmake**. Such directories from previous runs are also unconditionally cleaned at the beginning of any **cmake** execution.

This command supports an alternate signature for CMake older than 3.25. The signature above is recommended for clarity.

~~~
try_compile(<compileResultVar> <bindir> {<srcfile>|SOURCES <srcfile>...}
            [CMAKE_FLAGS <flags>...]
            [COMPILE_DEFINITIONS <defs>...]
            [LINK_OPTIONS <options>...]
            [LINK_LIBRARIES <libs>...]
            [OUTPUT_VARIABLE <var>]
            [COPY_FILE <fileName> [COPY_FILE_ERROR <var>]]
            [<LANG>_STANDARD <std>]
            [<LANG>_STANDARD_REQUIRED <bool>]
            [<LANG>_EXTENSIONS <bool>]
            )
~~~

In this version, `try_compile` will use `<bindir>/CMakeFiles/CMakeTmp` for its operation, and all such files will be cleaned automatically. For debugging, [`--debug-trycompile`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-debug-trycompile) can be passed to **cmake** to avoid this clean. However, multiple sequential `try_compile` operations, if given the same `<bindir>`, will reuse this single output directory, such that you can only debug one such `try_compile` call at a time. Use of the newer signature is recommended to simplify debugging of multiple `try_compile` operations.

## [Options](https://cmake.org/cmake/help/v4.4/command/try_compile.html#id6)

The options for the above signatures are:

**`CMAKE_FLAGS <flags>...`**

Specify flags of the form [`-DVAR:TYPE=VALUE`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-D) to be passed to the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command-line used to drive the test build. The above example shows how values for variables `COMPILE_DEFINITIONS`, `INCLUDE_DIRECTORIES`, `LINK_DIRECTORIES`, `LINK_LIBRARIES`, and `LINK_OPTIONS` are used. Compiler options can be passed in like `CMAKE_FLAGS -DCOMPILE_DEFINITIONS=-Werror`.

**`COMPILE_DEFINITIONS <defs>...`**

Specify `-Ddefinition` arguments to pass to [`add_definitions()`](https://cmake.org/cmake/help/v4.4/command/add_definitions.html#command:add_definitions) in the generated test project.

**`COPY_FILE <fileName>`**

Copy the built executable or static library to the given `<fileName>`.

**`COPY_FILE_ERROR <var>`**

Use after `COPY_FILE` to capture into variable `<var>` any error message encountered while trying to copy the file.

**`LINK_LIBRARIES <libs>...`**

Specify libraries to be linked in the generated project. The list of libraries may refer to system libraries and to [Imported Targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets) from the calling project.

If this option is specified, any `-DLINK_LIBRARIES=...` value given to the `CMAKE_FLAGS` option will be ignored.

Added in version 3.29: Alias targets to imported libraries are also supported.

**`LINK_OPTIONS <options>...`**

Added in version 3.14.

Specify link step options to pass to [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options) or to set the [`STATIC_LIBRARY_OPTIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/STATIC_LIBRARY_OPTIONS.html#prop_tgt:STATIC_LIBRARY_OPTIONS) target property in the generated project, depending on the [`CMAKE_TRY_COMPILE_TARGET_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_TARGET_TYPE.html#variable:CMAKE_TRY_COMPILE_TARGET_TYPE) variable.

**`LINKER_LANGUAGE <lang>`**

Added in version 3.29.

Specify the [`LINKER_LANGUAGE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINKER_LANGUAGE.html#prop_tgt:LINKER_LANGUAGE) target property of the generated project. When using multiple source files with different languages, set this to the language of the source file containing the program entry point, e.g., `main`.

**`LOG_DESCRIPTION <text>`**

Added in version 3.26.

Specify a non-empty text description of the purpose of the check. This is recorded in the [`cmake-configure-log(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-configure-log.7.html#manual:cmake-configure-log(7)) entry.

**`NO_CACHE`**

Added in version 3.25.

`<compileResultVar>` will be stored in a normal variable rather than a cache entry.

`<compileResultVar>` is normally cached so that a simple pattern can be used to avoid repeating the test on subsequent executions of CMake:

~~~
if(NOT DEFINED RESULTVAR)
  # ...(check-specific setup code)...
  try_compile(RESULTVAR ...)
  # ...(check-specific logging and cleanup code)...
endif()
~~~

If the guard variable and result variable are not the same (for example, if the test is part of a larger inspection), `NO_CACHE` may be useful to avoid leaking the intermediate result variable into the cache.

**`NO_LOG`**

Added in version 3.26.

Do not record a [`cmake-configure-log(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-configure-log.7.html#manual:cmake-configure-log(7)) entry for this call.

**`OUTPUT_VARIABLE <var>`**

Store the output from the build process in the given variable.

**`SOURCE_FROM_CONTENT <name> <content>`**

Added in version 3.25.

Write `<content>` to a file named `<name>` in the operation directory. This can be used to bypass the need to separately write a source file when the contents of the file are dynamically specified. The specified `<name>` is not allowed to contain path components.

`SOURCE_FROM_CONTENT` may be specified multiple times.

**`SOURCE_FROM_FILE <name> <path>`**

Added in version 3.25.

Copy `<path>` to a file named `<name>` in the operation directory. This can be used to consolidate files into the operation directory, which may be useful if a source which already exists (i.e. as a stand-alone file in a project's source repository) needs to refer to other file(s) created by `SOURCE_FROM_*`. (Otherwise, `SOURCES` is usually more convenient.) The specified `<name>` is not allowed to contain path components.

**`SOURCE_FROM_VAR <name> <var>`**

Added in version 3.25.

Write the contents of `<var>` to a file named `<name>` in the operation directory. This is the same as `SOURCE_FROM_CONTENT`, but takes the contents from the specified CMake variable, rather than directly, which may be useful when passing arguments through a function which wraps `try_compile`. The specified `<name>` is not allowed to contain path components.

`SOURCE_FROM_VAR` may be specified multiple times.

**`SOURCES_TYPE <type>`**

Added in version 3.28.

Sources may be classified using the `SOURCES_TYPE` argument. Once specified, all subsequent sources specified will be treated as that type until another `SOURCES_TYPE` is given. Available types are:

**`NORMAL`**

Sources are not added to any `FILE_SET` in the generated project.

**`CXX_MODULE`**

Added in version 3.28.

Sources are added to a `FILE_SET` of type `CXX_MODULES` in the generated project.

The default type of sources is `NORMAL`.

**`<LANG>_STANDARD <std>`**

Added in version 3.8.

Specify the [`C_STANDARD`](https://cmake.org/cmake/help/v4.4/prop_tgt/C_STANDARD.html#prop_tgt:C_STANDARD), [`CXX_STANDARD`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_STANDARD.html#prop_tgt:CXX_STANDARD), [`OBJC_STANDARD`](https://cmake.org/cmake/help/v4.4/prop_tgt/OBJC_STANDARD.html#prop_tgt:OBJC_STANDARD), [`OBJCXX_STANDARD`](https://cmake.org/cmake/help/v4.4/prop_tgt/OBJCXX_STANDARD.html#prop_tgt:OBJCXX_STANDARD), or [`CUDA_STANDARD`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_STANDARD.html#prop_tgt:CUDA_STANDARD) target property of the generated project.

**`<LANG>_STANDARD_REQUIRED <bool>`**

Added in version 3.8.

Specify the [`C_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/prop_tgt/C_STANDARD_REQUIRED.html#prop_tgt:C_STANDARD_REQUIRED), [`CXX_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_STANDARD_REQUIRED.html#prop_tgt:CXX_STANDARD_REQUIRED), [`OBJC_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/prop_tgt/OBJC_STANDARD_REQUIRED.html#prop_tgt:OBJC_STANDARD_REQUIRED), [`OBJCXX_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/prop_tgt/OBJCXX_STANDARD_REQUIRED.html#prop_tgt:OBJCXX_STANDARD_REQUIRED),or [`CUDA_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_STANDARD_REQUIRED.html#prop_tgt:CUDA_STANDARD_REQUIRED) target property of the generated project.

**`<LANG>_EXTENSIONS <bool>`**

Added in version 3.8.

Specify the [`C_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/C_EXTENSIONS.html#prop_tgt:C_EXTENSIONS), [`CXX_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/CXX_EXTENSIONS.html#prop_tgt:CXX_EXTENSIONS), [`OBJC_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/OBJC_EXTENSIONS.html#prop_tgt:OBJC_EXTENSIONS), [`OBJCXX_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/OBJCXX_EXTENSIONS.html#prop_tgt:OBJCXX_EXTENSIONS), or [`CUDA_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/prop_tgt/CUDA_EXTENSIONS.html#prop_tgt:CUDA_EXTENSIONS) target property of the generated project.

## [Other Behavior Settings](https://cmake.org/cmake/help/v4.4/command/try_compile.html#id7)

Added in version 3.4: If set, the following variables are passed in to the generated try_compile CMakeLists.txt to initialize compile target properties with default values:

- [`CMAKE_CUDA_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_RUNTIME_LIBRARY.html#variable:CMAKE_CUDA_RUNTIME_LIBRARY)

- [`CMAKE_ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ENABLE_EXPORTS.html#variable:CMAKE_ENABLE_EXPORTS)

- [`CMAKE_EXECUTABLE_ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXECUTABLE_ENABLE_EXPORTS.html#variable:CMAKE_EXECUTABLE_ENABLE_EXPORTS)

- [`CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS.html#variable:CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS)

- [`CMAKE_EXE_LINKER_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXE_LINKER_FLAGS.html#variable:CMAKE_EXE_LINKER_FLAGS), unless using CMake versions prior to 4.0 without policy [`CMP0056`](https://cmake.org/cmake/help/v4.4/policy/CMP0056.html#policy:CMP0056) set to `NEW`

- [`CMAKE_<LANG>_LINK_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS), unless using CMake versions prior to 4.3 without policy [`CMP0210`](https://cmake.org/cmake/help/v4.4/policy/CMP0210.html#policy:CMP0210) set to `NEW`

- [`CMAKE_LINK_SEARCH_START_STATIC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_SEARCH_START_STATIC.html#variable:CMAKE_LINK_SEARCH_START_STATIC)

- [`CMAKE_LINK_SEARCH_END_STATIC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_SEARCH_END_STATIC.html#variable:CMAKE_LINK_SEARCH_END_STATIC)

- [`CMAKE_MSVC_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_RUNTIME_LIBRARY.html#variable:CMAKE_MSVC_RUNTIME_LIBRARY)

- [`CMAKE_POSITION_INDEPENDENT_CODE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POSITION_INDEPENDENT_CODE.html#variable:CMAKE_POSITION_INDEPENDENT_CODE)

- [`CMAKE_WATCOM_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_WATCOM_RUNTIME_LIBRARY.html#variable:CMAKE_WATCOM_RUNTIME_LIBRARY)

Changed in version 3.14: If [`CMP0083`](https://cmake.org/cmake/help/v4.4/policy/CMP0083.html#policy:CMP0083) is set to `NEW`, then in order to obtain correct behavior at link time, the `check_pie_supported()` command from the [`CheckPIESupported`](https://cmake.org/cmake/help/v4.4/module/CheckPIESupported.html#module:CheckPIESupported) module must be called before using the `try_compile` command.

Some policies are set automatically in the generated test project as needed to honor the state of the calling project:

- [`CMP0065`](https://cmake.org/cmake/help/v4.4/policy/CMP0065.html#policy:CMP0065) (in CMake versions prior to 4.0)

- [`CMP0083`](https://cmake.org/cmake/help/v4.4/policy/CMP0083.html#policy:CMP0083)

- [`CMP0091`](https://cmake.org/cmake/help/v4.4/policy/CMP0091.html#policy:CMP0091)

- [`CMP0104`](https://cmake.org/cmake/help/v4.4/policy/CMP0104.html#policy:CMP0104)

- [`CMP0123`](https://cmake.org/cmake/help/v4.4/policy/CMP0123.html#policy:CMP0123)

- [`CMP0126`](https://cmake.org/cmake/help/v4.4/policy/CMP0126.html#policy:CMP0126)

- [`CMP0128`](https://cmake.org/cmake/help/v4.4/policy/CMP0128.html#policy:CMP0128)

- [`CMP0136`](https://cmake.org/cmake/help/v4.4/policy/CMP0136.html#policy:CMP0136)

- [`CMP0141`](https://cmake.org/cmake/help/v4.4/policy/CMP0141.html#policy:CMP0141)

- [`CMP0155`](https://cmake.org/cmake/help/v4.4/policy/CMP0155.html#policy:CMP0155)

- [`CMP0157`](https://cmake.org/cmake/help/v4.4/policy/CMP0157.html#policy:CMP0157)

- [`CMP0181`](https://cmake.org/cmake/help/v4.4/policy/CMP0181.html#policy:CMP0181)

- [`CMP0184`](https://cmake.org/cmake/help/v4.4/policy/CMP0184.html#policy:CMP0184)

- [`CMP0210`](https://cmake.org/cmake/help/v4.4/policy/CMP0210.html#policy:CMP0210)

Set variable [`CMAKE_TRY_COMPILE_CONFIGURATION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_CONFIGURATION.html#variable:CMAKE_TRY_COMPILE_CONFIGURATION) to choose a build configuration:

- For multi-config generators, this selects which configuration to build.

- For single-config generators, this sets [`CMAKE_BUILD_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BUILD_TYPE.html#variable:CMAKE_BUILD_TYPE) in the test project.

Added in version 3.6: Set the [`CMAKE_TRY_COMPILE_TARGET_TYPE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_TARGET_TYPE.html#variable:CMAKE_TRY_COMPILE_TARGET_TYPE) variable to specify the type of target used for the source file signature.

Added in version 3.6: Set the [`CMAKE_TRY_COMPILE_PLATFORM_VARIABLES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_PLATFORM_VARIABLES.html#variable:CMAKE_TRY_COMPILE_PLATFORM_VARIABLES) variable to specify variables that must be propagated into the test project. This variable is meant for use only in toolchain files and is only honored by the `try_compile()` command for the source files form, not when given a whole project.

Changed in version 3.8: If [`CMP0067`](https://cmake.org/cmake/help/v4.4/policy/CMP0067.html#policy:CMP0067) is set to `NEW`, or any of the `<LANG>_STANDARD`, `<LANG>_STANDARD_REQUIRED`, or `<LANG>_EXTENSIONS` options are used, then the language standard variables are honored:

- [`CMAKE_C_STANDARD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_C_STANDARD.html#variable:CMAKE_C_STANDARD)

- [`CMAKE_C_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_C_STANDARD_REQUIRED.html#variable:CMAKE_C_STANDARD_REQUIRED)

- [`CMAKE_C_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_C_EXTENSIONS.html#variable:CMAKE_C_EXTENSIONS)

- [`CMAKE_CXX_STANDARD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CXX_STANDARD.html#variable:CMAKE_CXX_STANDARD)

- [`CMAKE_CXX_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CXX_STANDARD_REQUIRED.html#variable:CMAKE_CXX_STANDARD_REQUIRED)

- [`CMAKE_CXX_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CXX_EXTENSIONS.html#variable:CMAKE_CXX_EXTENSIONS)

- [`CMAKE_OBJC_STANDARD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJC_STANDARD.html#variable:CMAKE_OBJC_STANDARD)

- [`CMAKE_OBJC_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJC_STANDARD_REQUIRED.html#variable:CMAKE_OBJC_STANDARD_REQUIRED)

- [`CMAKE_OBJC_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJC_EXTENSIONS.html#variable:CMAKE_OBJC_EXTENSIONS)

- [`CMAKE_OBJCXX_STANDARD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJCXX_STANDARD.html#variable:CMAKE_OBJCXX_STANDARD)

- [`CMAKE_OBJCXX_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJCXX_STANDARD_REQUIRED.html#variable:CMAKE_OBJCXX_STANDARD_REQUIRED)

- [`CMAKE_OBJCXX_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OBJCXX_EXTENSIONS.html#variable:CMAKE_OBJCXX_EXTENSIONS)

- [`CMAKE_CUDA_STANDARD`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_STANDARD.html#variable:CMAKE_CUDA_STANDARD)

- [`CMAKE_CUDA_STANDARD_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_STANDARD_REQUIRED.html#variable:CMAKE_CUDA_STANDARD_REQUIRED)

- [`CMAKE_CUDA_EXTENSIONS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_EXTENSIONS.html#variable:CMAKE_CUDA_EXTENSIONS)

Their values are used to set the corresponding target properties in the generated project (unless overridden by an explicit option).

Changed in version 3.14: For the [`Green Hills MULTI`](https://cmake.org/cmake/help/v4.4/generator/Green%20Hills%20MULTI.html#generator:Green%20Hills%20MULTI) generator, the GHS toolset and target system customization cache variables are also propagated into the test project.

Added in version 3.24: The [`CMAKE_TRY_COMPILE_NO_PLATFORM_VARIABLES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TRY_COMPILE_NO_PLATFORM_VARIABLES.html#variable:CMAKE_TRY_COMPILE_NO_PLATFORM_VARIABLES) variable may be set to disable passing platform variables into the test project.

Added in version 3.25: If [`CMP0141`](https://cmake.org/cmake/help/v4.4/policy/CMP0141.html#policy:CMP0141) is set to `NEW`, one can use [`CMAKE_MSVC_DEBUG_INFORMATION_FORMAT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_DEBUG_INFORMATION_FORMAT.html#variable:CMAKE_MSVC_DEBUG_INFORMATION_FORMAT) to specify the MSVC debug information format.

Added in version 3.30: If the [`PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE`](https://cmake.org/cmake/help/v4.4/prop_gbl/PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE.html#prop_gbl:PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE) global property is set to true, [`CMAKE_PROJECT_TOP_LEVEL_INCLUDES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_TOP_LEVEL_INCLUDES.html#variable:CMAKE_PROJECT_TOP_LEVEL_INCLUDES) is propagated into the test project's build configuration when using the [whole-project signature](https://cmake.org/cmake/help/v4.4/command/try_compile.html#try-compiling-whole-projects).

Added in version 4.0: If [`CMP0184`](https://cmake.org/cmake/help/v4.4/policy/CMP0184.html#policy:CMP0184) is set to `NEW`, one can use [`CMAKE_MSVC_RUNTIME_CHECKS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_RUNTIME_CHECKS.html#variable:CMAKE_MSVC_RUNTIME_CHECKS) to specify the enabled MSVC runtime checks.

Added in version 4.3: If [`CMP0210`](https://cmake.org/cmake/help/v4.4/policy/CMP0210.html#policy:CMP0210) is set to `NEW`, [`CMAKE_<LANG>_LINK_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS) and [`CMAKE_<LANG>_LINK_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS_%3CCONFIG%3E) are propagated into the test project's build configuration.

## [See Also](https://cmake.org/cmake/help/v4.4/command/try_compile.html#id8)

- [`try_run()`](https://cmake.org/cmake/help/v4.4/command/try_run.html#command:try_run)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
