CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html)

# add_custom_command

Add a custom build rule to the generated build system.

## Synopsis

~~~
Generating Files
  add_custom_command(OUTPUT <output1> [<output2> ...]
                     COMMAND <command1> [ARGS] [<args1>...]
                     [...])

Build Events
  add_custom_command(TARGET <target>
                     PRE_BUILD | PRE_LINK | POST_BUILD
                     [...])
~~~

## Generating Files

**add_custom_command(OUTPUT \<output1\> [\<output2\> ...] COMMAND \<command1\> [ARGS] [\<args1\>...] [...])**

Add a custom command to produce an output:

~~~
add_custom_command(OUTPUT <output1> [<output2> ...]
                   COMMAND <command1> [ARGS] [<args1>...]
                   [COMMAND <command2> [ARGS] [<args2>...]] ...
                   [MAIN_DEPENDENCY depend]
                   [DEPENDS <depends>...]
                   [BYPRODUCTS <files>...]
                   [IMPLICIT_DEPENDS <lang1> <depend1>
                                    [<lang2> <depend2>] ...]
                   [WORKING_DIRECTORY <dir>]
                   [COMMENT <comment>]
                   [DEPFILE <depfile>]
                   [JOB_POOL <job_pool>]
                   [JOB_SERVER_AWARE <bool>]
                   [VERBATIM] [APPEND] [USES_TERMINAL]
                   [CODEGEN]
                   [COMMAND_EXPAND_LISTS]
                   [DEPENDS_EXPLICIT_ONLY])
~~~

This defines a command to generate specified `OUTPUT` file(s). A target created in the same directory (`CMakeLists.txt` file) that specifies any output of the custom command as a source file is given a rule to generate the file using the command at build time.

Do not list the output in more than one independent target that may build in parallel or the instances of the rule may conflict. Instead, use the [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) command to drive the command and make the other targets depend on that one. See the [Example: Generating Files for Multiple Targets](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#example-generating-files-for-multiple-targets) below.

The options are:

**`APPEND`**

Append the `COMMAND` and `DEPENDS` option values to the custom command for the first output specified. There must have already been a previous call to this command with the same output.

If the previous call specified the output via a generator expression, the output specified by the current call must match in at least one configuration after evaluating generator expressions. In this case, the appended commands and dependencies apply to all configurations.

The `COMMENT`, `MAIN_DEPENDENCY`, and `WORKING_DIRECTORY` options are currently ignored when `APPEND` is given, but may be used in the future.

**`BYPRODUCTS`**

Added in version 3.2.

Specify the files the command is expected to produce but whose modification time may or may not be newer than the dependencies. If a byproduct name is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory. Each byproduct file will be marked with the [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) source file property automatically.

*See policy* [`CMP0058`](https://cmake.org/cmake/help/v4.4/policy/CMP0058.html#policy:CMP0058) *for the motivation behind this feature.*

Explicit specification of byproducts is supported by the [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator to tell the `ninja` build tool how to regenerate byproducts when they are missing. It is also useful when other build rules (e.g. custom commands) depend on the byproducts. Ninja requires a build rule for any generated file on which another rule depends even if there are order-only dependencies to ensure the byproducts will be available before their dependents build.

The [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) will remove `BYPRODUCTS` and other [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) files during `make clean`.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). All byproducts must be set in the first call to `add_custom_command(OUTPUT...)` for the output files.

Added in version 3.20: Arguments to `BYPRODUCTS` may use a restricted set of [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)). [Target-dependent expressions](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#target-dependent-expressions) are not permitted.

Changed in version 3.28: In targets using [File Sets](https://cmake.org/cmake/help/v4.4/command/target_sources.html#file-sets), custom command byproducts are now considered private unless they are listed in a non-private file set. See policy [`CMP0154`](https://cmake.org/cmake/help/v4.4/policy/CMP0154.html#policy:CMP0154).

**`COMMAND`**

Specify the command-line(s) to execute at build time. At least one `COMMAND` would normally be given, but certain patterns may omit it, such as adding commands in separate calls using `APPEND`.

If more than one `COMMAND` is specified, they will be executed in order, but *not* necessarily composed into a stateful shell or batch script. To run a full script, use the [`configure_file()`](https://cmake.org/cmake/help/v4.4/command/configure_file.html#command:configure_file) command or the [`file(GENERATE)`](https://cmake.org/cmake/help/v4.4/command/file.html#generate) command to create it, and then specify a `COMMAND` to launch it.

The optional `ARGS` argument is for backward compatibility and will be ignored.

If `COMMAND` specifies an executable target name (created by the [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) command), it will automatically be replaced by the location of the executable created at build time if either of the following is true:

- The target is not being cross-compiled (i.e. the [`CMAKE_CROSSCOMPILING`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSSCOMPILING.html#variable:CMAKE_CROSSCOMPILING) variable is not set to true).

- Added in version 3.6: The target is being cross-compiled and an emulator is provided (i.e. its [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) target property is set). In this case, the contents of [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) will be prepended to the command before the location of the target executable.

If neither of the above conditions are met, it is assumed that the command name is a program to be found on the `PATH` at build time.

Arguments to `COMMAND` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)). Use the [`TARGET_FILE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:TARGET_FILE) generator expression to refer to the location of a target later in the command line (i.e. as a command argument rather than as the command to execute).

Whenever one of the following target based generator expressions are used as a command to execute or is mentioned in a command argument, a target-level dependency will be added automatically so that the mentioned target will be built before any target using this custom command (see policy [`CMP0112`](https://cmake.org/cmake/help/v4.4/policy/CMP0112.html#policy:CMP0112)).

- `TARGET_FILE`

- `TARGET_LINKER_FILE`

- `TARGET_SONAME_FILE`

- `TARGET_PDB_FILE`

This target-level dependency does NOT add a file-level dependency that would cause the custom command to re-run whenever the executable is recompiled. List target names with the `DEPENDS` option to add such file-level dependencies.

**`COMMENT`**

Display the given message before the commands are executed at build time. This will be ignored if `APPEND` is given, although a future version may use it.

Added in version 3.26: Arguments to `COMMENT` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

**`DEPENDS`**

Specify files on which the command depends. Each argument is converted to a dependency as follows:

- If the argument is the name of a target (created by the [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target), [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable), or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) command) a target-level dependency is created to make sure the target is built before any target using this custom command. Additionally, if the target is an executable or library, a file-level dependency is created to cause the custom command to re-run whenever the target is recompiled.

    

  Changed in version 4.4: Previously an executable target could be specified using the name of the target with `.exe` appended. This is no longer allowed. See policy [`CMP0212`](https://cmake.org/cmake/help/v4.4/policy/CMP0212.html#policy:CMP0212).

- If the argument is an absolute path, a file-level dependency is created on that path.

- If the argument is the name of a source file that has been added to a target or on which a source file property has been set, a file-level dependency is created on that source file.

- If the argument is a relative path and it exists in the current source directory, a file-level dependency is created on that file in the current source directory.

- Otherwise, a file-level dependency is created on that path relative to the current binary directory.

If any dependency is an `OUTPUT` of another custom command in the same directory (`CMakeLists.txt` file), CMake automatically brings the other custom command into the target in which this command is built.

Added in version 3.16: A target-level dependency is added if any dependency is listed as `BYPRODUCTS` of a target or any of its build events in the same directory to ensure the byproducts will be available.

If `DEPENDS` is not specified, the command will run whenever the `OUTPUT` is missing; if the command does not actually create the `OUTPUT`, the rule will always run.

Added in version 3.1: Arguments to `DEPENDS` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

**`COMMAND_EXPAND_LISTS`**

Added in version 3.8.

Lists in `COMMAND` arguments will be expanded, including those created with [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)), allowing `COMMAND` arguments such as `${CC} "-I$<JOIN:$<TARGET_PROPERTY:foo,INCLUDE_DIRECTORIES>,;-I>" foo.cc` to be properly expanded.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). If the appended commands need this option to be set, it must be set on the first call to `add_custom_command(OUTPUT...)` for the output files.

**`CODEGEN`**

Added in version 3.31.

Adds the custom command to a global `codegen` target that can be used to execute the custom command while avoiding the majority of the build graph.

This option is supported only by [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators) and [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators), and is ignored by other generators. Furthermore, this option is allowed only if policy [`CMP0171`](https://cmake.org/cmake/help/v4.4/policy/CMP0171.html#policy:CMP0171) is set to `NEW`.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). It can only be set on the first call to `add_custom_command(OUTPUT...)` for the output files.

**`IMPLICIT_DEPENDS`**

Request scanning of implicit dependencies of an input file. The language given specifies the programming language whose corresponding dependency scanner should be used. Currently only `C` and `CXX` language scanners are supported. The language has to be specified for every file in the `IMPLICIT_DEPENDS` list. Dependencies discovered from the scanning are added to those of the custom command at build time. Note that the `IMPLICIT_DEPENDS` option is currently supported only for Makefile generators and will be ignored by other generators.

Note

This option cannot be specified at the same time as `DEPFILE` option.

**`JOB_POOL`**

Added in version 3.15.

Specify a [`pool`](https://cmake.org/cmake/help/v4.4/prop_gbl/JOB_POOLS.html#prop_gbl:JOB_POOLS) for the [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator. Incompatible with `USES_TERMINAL`, which implies the `console` pool. Using a pool that is not defined by [`JOB_POOLS`](https://cmake.org/cmake/help/v4.4/prop_gbl/JOB_POOLS.html#prop_gbl:JOB_POOLS) causes an error by ninja at build time.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). Job pools can only be specified in the first call to `add_custom_command(OUTPUT...)` for the output files.

**`JOB_SERVER_AWARE`**

Added in version 3.28.

Specify that the command is GNU Make job server aware.

For the [`Unix Makefiles`](https://cmake.org/cmake/help/v4.4/generator/Unix%20Makefiles.html#generator:Unix%20Makefiles), [`MSYS Makefiles`](https://cmake.org/cmake/help/v4.4/generator/MSYS%20Makefiles.html#generator:MSYS%20Makefiles), and [`MinGW Makefiles`](https://cmake.org/cmake/help/v4.4/generator/MinGW%20Makefiles.html#generator:MinGW%20Makefiles) generators this will add the `+` prefix to the recipe line. See the [GNU Make Documentation](https://www.gnu.org/software/make/manual/html_node/MAKE-Variable.html) for more information.

This option is silently ignored by other generators.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). Job server awareness can only be specified in the first call to `add_custom_command(OUTPUT...)` for the output files.

**`MAIN_DEPENDENCY`**

Specify the primary input source file to the command. This is treated just like any value given to the `DEPENDS` option but also suggests to [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) where to hang the custom command. Each source file may have at most one command specifying it as its main dependency. A compile command (i.e. for a library or an executable) counts as an implicit main dependency which gets silently overwritten by a custom command specification.

This option is currently ignored if `APPEND` is given, but a future version may use it.

**`OUTPUT`**

Specify the output files the command is expected to produce. Each output file will be marked with the [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) source file property automatically. If the output of the custom command is not actually created as a file on disk it should be marked with the [`SYMBOLIC`](https://cmake.org/cmake/help/v4.4/prop_sf/SYMBOLIC.html#prop_sf:SYMBOLIC) source file property.

If an output file name is a relative path, its absolute path is determined by interpreting it relative to:

- the build directory corresponding to the current source directory ([`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR)), or

- the current source directory ([`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR)).

The path in the build directory is preferred unless the path in the source tree is mentioned as an absolute source file path elsewhere in the current directory.

The output file path may not contain `<` or `>` characters.

Added in version 3.20: Arguments to `OUTPUT` may use a restricted set of [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)). [Target-dependent expressions](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#target-dependent-expressions) are not permitted.

Changed in version 3.28: In targets using [File Sets](https://cmake.org/cmake/help/v4.4/command/target_sources.html#file-sets), custom command outputs are now considered private unless they are listed in a non-private file set. See policy [`CMP0154`](https://cmake.org/cmake/help/v4.4/policy/CMP0154.html#policy:CMP0154).

Changed in version 3.30: The output file path may now use `#` characters, except when using the [`Borland Makefiles`](https://cmake.org/cmake/help/v4.4/generator/Borland%20Makefiles.html#generator:Borland%20Makefiles) generator.

**`USES_TERMINAL`**

Added in version 3.2.

The command will be given direct access to the terminal if possible. With the [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator, this places the command in the `console` [`pool`](https://cmake.org/cmake/help/v4.4/prop_gbl/JOB_POOLS.html#prop_gbl:JOB_POOLS).

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). If the appended commands need access to the terminal, it must be set on the first call to `add_custom_command(OUTPUT...)` for the output files.

**`VERBATIM`**

All arguments to the commands will be escaped properly for the build tool so that the invoked command receives each argument unchanged. Note that one level of escapes is still used by the CMake language processor before add_custom_command even sees the arguments. Use of `VERBATIM` is recommended as it enables correct behavior. When `VERBATIM` is not given the behavior is platform specific because there is no protection of tool-specific special characters.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). If the appended commands need to be treated as `VERBATIM`, it must be set on the first call to `add_custom_command(OUTPUT...)` for the output files.

**`WORKING_DIRECTORY`**

Execute the command with the given current working directory. If it is a relative path, it will be interpreted relative to the build tree directory corresponding to the current source directory. If not specified, set to [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR).

This option is currently ignored if `APPEND` is given, but a future version may use it.

Added in version 3.13: Arguments to `WORKING_DIRECTORY` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

**`DEPFILE`**

Added in version 3.7.

Specify a depfile which holds dependencies for the custom command. It is usually emitted by the custom command itself. This keyword may only be used if the generator supports it, as detailed below.

The expected format, compatible with what is generated by `gcc` with the option `-M`, is independent of the generator or platform.

The formal syntax, as specified using [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) notation with the regular extensions, is the following:

~~~
depfile       ::= rule*
rule          ::= targets (':' (separator dependencies?)?)? eol
targets       ::= target (separator target)* separator*
target        ::= pathname
dependencies  ::= dependency (separator dependency)* separator*
dependency    ::= pathname
separator     ::= (space | line_continue)+
line_continue ::= '\' eol
space         ::= ' ' | '\t'
pathname      ::= character+
character     ::= std_character | dollar | hash | whitespace
std_character ::= <any character except '$', '#' or ' '>
dollar        ::= '$$'
hash          ::= '\#'
whitespace    ::= '\ '
eol           ::= '\r'? '\n'
~~~

Note

As part of `pathname`, any slash and backslash is interpreted as a directory separator.

Added in version 3.7: The [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator supports `DEPFILE` since the keyword was first added.

Added in version 3.17: Added the [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config) generator, which included support for the `DEPFILE` keyword.

Added in version 3.20: Added support for [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators).

Note

`DEPFILE` cannot be specified at the same time as the `IMPLICIT_DEPENDS` option for [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators).

Added in version 3.21: Added support for [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) with VS 2012 and above, and for the [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator. Support for [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) was also added.

Added in version 3.29: The [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators) will now incorporate the dependencies into its "deps log" database if the file is not listed in `OUTPUTS` or `BYPRODUCTS`.

Using `DEPFILE` with generators other than those listed above is an error.

If the `DEPFILE` argument is relative, it should be relative to [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR), and any relative paths inside the `DEPFILE` should also be relative to [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR). See policy [`CMP0116`](https://cmake.org/cmake/help/v4.4/policy/CMP0116.html#policy:CMP0116), which is always `NEW` for [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators), [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators), and the [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). Depfiles can only be set on the first call to `add_custom_command(OUTPUT...)` for the output files.

`DEPENDS_EXPLICIT_ONLY`

Added in version 3.27.

Indicates that the command's `DEPENDS` argument represents all files required by the command and implicit dependencies are not required.

Without this option, if any target uses the output of the custom command, CMake will consider that target's dependencies as implicit dependencies for the custom command in case this custom command requires files implicitly created by those targets.

This option can be enabled on all custom commands by setting [`CMAKE_ADD_CUSTOM_COMMAND_DEPENDS_EXPLICIT_ONLY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ADD_CUSTOM_COMMAND_DEPENDS_EXPLICIT_ONLY.html#variable:CMAKE_ADD_CUSTOM_COMMAND_DEPENDS_EXPLICIT_ONLY) to `ON`.

This keyword cannot be used with `APPEND` (see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175)). It can only be set on the first call to `add_custom_command(OUTPUT...)` for the output files.

Only the [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators) actually use this information to remove unnecessary implicit dependencies.

See also the [`OPTIMIZE_DEPENDENCIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/OPTIMIZE_DEPENDENCIES.html#prop_tgt:OPTIMIZE_DEPENDENCIES) target property, which may provide another way for reducing the impact of target dependencies in some scenarios.

## Examples: Generating Files

Custom commands may be used to generate source files. For example, the code:

~~~
add_custom_command(
  OUTPUT out.c
  COMMAND someTool -i ${CMAKE_CURRENT_SOURCE_DIR}/in.txt
                   -o out.c
  DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/in.txt
  VERBATIM)
add_library(myLib out.c)
~~~

adds a custom command to run `someTool` to generate `out.c` and then compile the generated source as part of a library. The generation rule will re-run whenever `in.txt` changes.

Added in version 3.20: One may use generator expressions to specify per-configuration outputs. For example, the code:

~~~
add_custom_command(
  OUTPUT "out-$<CONFIG>.c"
  COMMAND someTool -i ${CMAKE_CURRENT_SOURCE_DIR}/in.txt
                   -o "out-$<CONFIG>.c"
                   -c "$<CONFIG>"
  DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/in.txt
  VERBATIM)
add_library(myLib "out-$<CONFIG>.c")
~~~

adds a custom command to run `someTool` to generate `out-<config>.c`, where `<config>` is the build configuration, and then compile the generated source as part of a library.

Added in version 3.31: Use the `CODEGEN` option to add a custom command's outputs to the builtin `codegen` target. This is useful to make generated code available for static analysis without building the entire project. For example:

~~~
add_executable(someTool someTool.c)

add_custom_command(
  OUTPUT out.c
  COMMAND someTool -o out.c
  CODEGEN)

add_library(myLib out.c)
~~~

A user may build the `codegen` target to generate `out.c`. `someTool` is built as dependency, but `myLib` is not built at all.

### Example: Generating Files for Multiple Targets

If multiple independent targets need the same custom command output, it must be attached to a single custom target on which they all depend. Consider the following example:

~~~
add_custom_command(
  OUTPUT table.csv
  COMMAND makeTable -i ${CMAKE_CURRENT_SOURCE_DIR}/input.dat
                    -o table.csv
  DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/input.dat
  VERBATIM)
add_custom_target(generate_table_csv DEPENDS table.csv)

add_custom_command(
  OUTPUT foo.cxx
  COMMAND genFromTable -i table.csv -case foo -o foo.cxx
  DEPENDS table.csv           # file-level dependency
          generate_table_csv  # target-level dependency
  VERBATIM)
add_library(foo foo.cxx)

add_custom_command(
  OUTPUT bar.cxx
  COMMAND genFromTable -i table.csv -case bar -o bar.cxx
  DEPENDS table.csv           # file-level dependency
          generate_table_csv  # target-level dependency
  VERBATIM)
add_library(bar bar.cxx)
~~~

Output `foo.cxx` is needed only by target `foo` and output `bar.cxx` is needed only by target `bar`, but *both* targets need `table.csv`, transitively. Since `foo` and `bar` are independent targets that may build concurrently, we prevent them from racing to generate `table.csv` by placing its custom command in a separate target, `generate_table_csv`. The custom commands generating `foo.cxx` and `bar.cxx` each specify a target-level dependency on `generate_table_csv`, so the targets using them, `foo` and `bar`, will not build until after target `generate_table_csv` is built.

## Build Events

**add_custom_command(TARGET \<target\> PRE_BUILD | PRE_LINK | POST_BUILD [...])**

Add a build event to a target such as a library or executable. This is useful for performing an operation before or after building the target. The command becomes part of the target and will only execute when the target itself is built. If the target is already built, the command will not execute.

~~~
add_custom_command(TARGET <target>
                   PRE_BUILD | PRE_LINK | POST_BUILD
                   COMMAND <command1> [ARGS] [<args1>...]
                   [COMMAND <command2> [ARGS] [<args2>...]] ...
                   [BYPRODUCTS <files>...]
                   [WORKING_DIRECTORY <dir>]
                   [COMMENT <comment>]
                   [VERBATIM]
                   [COMMAND_EXPAND_LISTS]
                   [USES_TERMINAL])
~~~

This defines a new command that will be associated with building the specified `<target>`. The `<target>` must be defined in the current directory; targets defined in other directories may not be specified.

When the command will happen is determined by which of the following is specified:

**`PRE_BUILD`**

This option has unique behavior for the [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators). When using one of the Visual Studio generators, the command will run before any other rules are executed within the target. With all other generators, this option behaves the same as `PRE_LINK` instead. Because of this, it is recommended to avoid using `PRE_BUILD` except when it is known that a Visual Studio generator is being used.

**`PRE_LINK`**

Run after sources have been compiled but before linking the binary or running the librarian or archiver tool of a static library. This is not defined for targets created by the [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) command.

**`POST_BUILD`**

Run after all other rules within the target have been executed.

Projects should always specify one of the above three keywords when using the `TARGET` form. See policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175).

All other keywords shown in the signature above have the same meaning as they do for the [`add_custom_command(OUTPUT)`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#output) form of the command. At least one `COMMAND` must be given, see policy [`CMP0175`](https://cmake.org/cmake/help/v4.4/policy/CMP0175.html#policy:CMP0175).

Note

Because generator expressions can be used in custom commands, it is possible to define `COMMAND` lines or whole custom commands which evaluate to empty strings for certain configurations. For [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) these command lines or custom commands will be omitted for the specific configuration and no "empty-string-command" will be added.

This allows adding individual build events for every configuration.

Added in version 3.21: Support for target-dependent generator expressions.

Added in version 3.29: The `<target>` may be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

## Examples: Build Events

A `POST_BUILD` event may be used to post-process a binary after linking. For example, the code:

~~~
add_executable(myExe myExe.c)
add_custom_command(
  TARGET myExe POST_BUILD
  COMMAND someHasher -i "$<TARGET_FILE:myExe>"
                     -o "$<TARGET_FILE:myExe>.hash"
  VERBATIM)
~~~

will run `someHasher` to produce a `.hash` file next to the executable after linking.

Added in version 3.20: One may use generator expressions to specify per-configuration byproducts. For example, the code:

~~~
add_library(myPlugin MODULE myPlugin.c)
add_custom_command(
  TARGET myPlugin POST_BUILD
  COMMAND someHasher -i "$<TARGET_FILE:myPlugin>"
                     --as-code "myPlugin-hash-$<CONFIG>.c"
  BYPRODUCTS "myPlugin-hash-$<CONFIG>.c"
  VERBATIM)
add_executable(myExe myExe.c "myPlugin-hash-$<CONFIG>.c")
~~~

will run `someHasher` after linking `myPlugin`, e.g. to produce a `.c` file containing code to check the hash of `myPlugin` that the `myExe` executable can use to verify it before loading.

## Ninja Multi-Config

Added in version 3.20: `add_custom_command` supports the [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config) generator's cross-config capabilities. See the generator documentation for more information.

## See Also

- [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
