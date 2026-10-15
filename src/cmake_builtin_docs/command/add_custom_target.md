CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html)

# add_custom_target

Add a target with no output so it will always be built.

~~~
add_custom_target(Name [ALL] [command1 [<args1>...]]
                  [COMMAND command2 [<args2>...]] ...
                  [DEPENDS <depend>...]
                  [BYPRODUCTS <file>...]
                  [WORKING_DIRECTORY <dir>]
                  [COMMENT <comment>]
                  [JOB_POOL <job_pool>]
                  [JOB_SERVER_AWARE <bool>]
                  [VERBATIM] [USES_TERMINAL]
                  [COMMAND_EXPAND_LISTS]
                  [SOURCES <source>...])
~~~

Adds a target with the given name that executes the given commands. The target has no output file and is *always considered out of date* even if the commands try to create a file with the name of the target. Use the [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) command to generate a file with dependencies. By default nothing depends on the custom target. Use the [`add_dependencies()`](https://cmake.org/cmake/help/v4.4/command/add_dependencies.html#command:add_dependencies) command to add dependencies to or from other targets.

The options are:

**`ALL`**

Indicate that this target should be added to the default build target so that it will be run every time (the command cannot be called `ALL`).

**`BYPRODUCTS`**

Added in version 3.2.

Specify the files the command is expected to produce but whose modification time may or may not be updated on subsequent builds. If a byproduct name is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory. Each byproduct file will be marked with the [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) source file property automatically.

*See policy* [`CMP0058`](https://cmake.org/cmake/help/v4.4/policy/CMP0058.html#policy:CMP0058) *for the motivation behind this feature.*

Explicit specification of byproducts is supported by the [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator to tell the `ninja` build tool how to regenerate byproducts when they are missing. It is also useful when other build rules (e.g. custom commands) depend on the byproducts. Ninja requires a build rule for any generated file on which another rule depends even if there are order-only dependencies to ensure the byproducts will be available before their dependents build.

The [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) will remove `BYPRODUCTS` and other [`GENERATED`](https://cmake.org/cmake/help/v4.4/prop_sf/GENERATED.html#prop_sf:GENERATED) files during `make clean`.

Added in version 3.20: Arguments to `BYPRODUCTS` may use a restricted set of [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)). [Target-dependent expressions](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#target-dependent-expressions) are not permitted.

Changed in version 3.28: In custom targets using [File Sets](https://cmake.org/cmake/help/v4.4/command/target_sources.html#file-sets), byproducts are now considered private unless they are listed in a non-private file set. See policy [`CMP0154`](https://cmake.org/cmake/help/v4.4/policy/CMP0154.html#policy:CMP0154).

**`COMMAND`**

Specify the command-line(s) to execute at build time. If more than one `COMMAND` is specified they will be executed in order, but *not* necessarily composed into a stateful shell or batch script. (To run a full script, use the [`configure_file()`](https://cmake.org/cmake/help/v4.4/command/configure_file.html#command:configure_file) command or the [`file(GENERATE)`](https://cmake.org/cmake/help/v4.4/command/file.html#generate) command to create it, and then specify a `COMMAND` to launch it.)

If `COMMAND` specifies an executable target name (created by the [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) command), it will automatically be replaced by the location of the executable created at build time if either of the following is true:

- The target is not being cross-compiled (i.e. the [`CMAKE_CROSSCOMPILING`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CROSSCOMPILING.html#variable:CMAKE_CROSSCOMPILING) variable is not set to true).

- Added in version 3.6: The target is being cross-compiled and an emulator is provided (i.e. its [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) target property is set). In this case, the contents of [`CROSSCOMPILING_EMULATOR`](https://cmake.org/cmake/help/v4.4/prop_tgt/CROSSCOMPILING_EMULATOR.html#prop_tgt:CROSSCOMPILING_EMULATOR) will be prepended to the command before the location of the target executable.

If neither of the above conditions are met, it is assumed that the command name is a program to be found on the `PATH` at build time.

Arguments to `COMMAND` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)). Use the [`TARGET_FILE`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:TARGET_FILE) generator expression to refer to the location of a target later in the command line (i.e. as a command argument rather than as the command to execute).

Whenever one of the following target based generator expressions are used as a command to execute or is mentioned in a command argument, a target-level dependency will be added automatically so that the mentioned target will be built before this custom target (see policy [`CMP0112`](https://cmake.org/cmake/help/v4.4/policy/CMP0112.html#policy:CMP0112)).

- `TARGET_FILE`

- `TARGET_LINKER_FILE`

- `TARGET_SONAME_FILE`

- `TARGET_PDB_FILE`

The command and arguments are optional and if not specified an empty target will be created.

**`COMMENT`**

Display the given message before the commands are executed at build time.

Added in version 3.26: Arguments to `COMMENT` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

**`DEPENDS`**

Reference files and outputs of custom commands created with [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) command calls in the same directory (`CMakeLists.txt` file). They will be brought up to date when the target is built.

Changed in version 3.16: A target-level dependency is added if any dependency is a byproduct of a target or any of its build events in the same directory to ensure the byproducts will be available before this target is built.

Use the [`add_dependencies()`](https://cmake.org/cmake/help/v4.4/command/add_dependencies.html#command:add_dependencies) command to add dependencies on other targets.

**`COMMAND_EXPAND_LISTS`**

Added in version 3.8.

Lists in `COMMAND` arguments will be expanded, including those created with [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)), allowing `COMMAND` arguments such as `${CC} "-I$<JOIN:$<TARGET_PROPERTY:foo,INCLUDE_DIRECTORIES>,;-I>" foo.cc` to be properly expanded.

**`JOB_POOL`**

Added in version 3.15.

Specify a [`pool`](https://cmake.org/cmake/help/v4.4/prop_gbl/JOB_POOLS.html#prop_gbl:JOB_POOLS) for the [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator. Incompatible with `USES_TERMINAL`, which implies the `console` pool. Using a pool that is not defined by [`JOB_POOLS`](https://cmake.org/cmake/help/v4.4/prop_gbl/JOB_POOLS.html#prop_gbl:JOB_POOLS) causes an error by ninja at build time.

**`JOB_SERVER_AWARE`**

Added in version 3.28.

Specify that the command is GNU Make job server aware.

For the [`Unix Makefiles`](https://cmake.org/cmake/help/v4.4/generator/Unix%20Makefiles.html#generator:Unix%20Makefiles), [`MSYS Makefiles`](https://cmake.org/cmake/help/v4.4/generator/MSYS%20Makefiles.html#generator:MSYS%20Makefiles), and [`MinGW Makefiles`](https://cmake.org/cmake/help/v4.4/generator/MinGW%20Makefiles.html#generator:MinGW%20Makefiles) generators this will add the `+` prefix to the recipe line. See the [GNU Make Documentation](https://www.gnu.org/software/make/manual/html_node/MAKE-Variable.html) for more information.

This option is silently ignored by other generators.

**`SOURCES`**

Specify additional source files to be included in the custom target. Specified source files will be added to IDE project files for convenience in editing even if they have no build rules.

**`VERBATIM`**

All arguments to the commands will be escaped properly for the build tool so that the invoked command receives each argument unchanged. Note that one level of escapes is still used by the CMake language processor before `add_custom_target` even sees the arguments. Use of `VERBATIM` is recommended as it enables correct behavior. When `VERBATIM` is not given the behavior is platform specific because there is no protection of tool-specific special characters.

**`USES_TERMINAL`**

Added in version 3.2.

The command will be given direct access to the terminal if possible. With the [`Ninja`](https://cmake.org/cmake/help/v4.4/generator/Ninja.html#generator:Ninja) generator, this places the command in the `console` [`pool`](https://cmake.org/cmake/help/v4.4/prop_gbl/JOB_POOLS.html#prop_gbl:JOB_POOLS).

**`WORKING_DIRECTORY`**

Execute the command with the given current working directory. If it is a relative path it will be interpreted relative to the build tree directory corresponding to the current source directory. If not specified, set to [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR).

Added in version 3.13: Arguments to `WORKING_DIRECTORY` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

## Ninja Multi-Config

Added in version 3.20: `add_custom_target` supports the [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config) generator's cross-config capabilities. See the generator documentation for more information.

## See Also

- [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
