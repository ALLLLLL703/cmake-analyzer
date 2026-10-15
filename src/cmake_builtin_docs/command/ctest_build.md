CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_build.html)

# ctest_build

Perform the [CTest Build Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-build-step) as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_build([BUILD <build-dir>] [APPEND]
            [CONFIGURATION <config>]
            [PARALLEL_LEVEL <parallel>]
            [FLAGS <flags>]
            [PROJECT_NAME <project-name>]
            [TARGET <target-name>]
            [NUMBER_ERRORS <num-err-var>]
            [NUMBER_WARNINGS <num-warn-var>]
            [RETURN_VALUE <result-var>]
            [CAPTURE_CMAKE_ERROR <result-var>]
            [PRESET <preset>]
            [PRESETS_FILE <file>]
            )
~~~

Build the project and store results in `Build.xml` for submission with the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command.

The [`CTEST_BUILD_COMMAND`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BUILD_COMMAND.html#variable:CTEST_BUILD_COMMAND) variable may be set to explicitly specify the build command line when no preset is selected. Otherwise the build command line is computed automatically based on the options given.

The options are:

**`BUILD <build-dir>`**

Specify the top-level build directory. If not given, the [`CTEST_BINARY_DIRECTORY`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BINARY_DIRECTORY.html#variable:CTEST_BINARY_DIRECTORY) variable is used.

**`APPEND`**

Mark `Build.xml` for append to results previously submitted to a dashboard server since the last [`ctest_start()`](https://cmake.org/cmake/help/v4.4/command/ctest_start.html#command:ctest_start) call. Append semantics are defined by the dashboard server in use. This does *not* cause results to be appended to a `.xml` file produced by a previous call to this command.

**`CONFIGURATION <config>`**

Specify the build configuration (e.g. `Debug`). If not specified the `CTEST_BUILD_CONFIGURATION` variable will be checked. Otherwise the [`-C <cfg>`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#cmdoption-ctest-C) option given to the [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) command will be used, if any.

**`PARALLEL_LEVEL <parallel>`**

Added in version 3.21.

Specify the parallel level of the underlying build system. If not specified, the [`CMAKE_BUILD_PARALLEL_LEVEL`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_BUILD_PARALLEL_LEVEL.html#envvar:CMAKE_BUILD_PARALLEL_LEVEL) environment variable will be checked.

**`FLAGS <flags>`**

Pass additional arguments to the underlying build command. If not specified the `CTEST_BUILD_FLAGS` variable will be checked. This can, e.g., be used to trigger a parallel build using the `-j` option of `make`. See the [`ProcessorCount`](https://cmake.org/cmake/help/v4.4/module/ProcessorCount.html#module:ProcessorCount) module for an example.

**`PROJECT_NAME <project-name>`**

Ignored since CMake 3.0.

Changed in version 3.14: This value is no longer required.

**`TARGET <target-name>`**

Specify the name of a target to build. If not specified the `CTEST_BUILD_TARGET` variable will be checked. Otherwise the default target will be built. This is the "all" target (called `ALL_BUILD` in [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators)).

**`NUMBER_ERRORS <num-err-var>`**

Store the number of build errors detected in the given variable.

**`NUMBER_WARNINGS <num-warn-var>`**

Store the number of build warnings detected in the given variable.

**`PRESET <preset>`**

Added in version 4.4.

Specify a [`preset`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#manual:cmake-presets(7)) to use when building the project. Any value set in the CTest script will take priority over a corresponding setting from the preset. For example, the `TARGET` argument will override the [`targets`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-buildpresets-targets) setting from the chosen preset.

When a preset is specified, the [`CTEST_BUILD_COMMAND`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BUILD_COMMAND.html#variable:CTEST_BUILD_COMMAND) variable is ignored so that the preset is always honored.

See also the [`CTEST_BUILD_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BUILD_PRESET.html#variable:CTEST_BUILD_PRESET) and [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET) variables.

**`PRESETS_FILE <file>`**

Added in version 4.4.

Specify a [`presets`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#manual:cmake-presets(7)) file to use instead of the default `CMakePresets.json` in the source directory. A relative path is interpreted relative to the source directory. Has no effect unless a preset is selected via the `PRESET` argument or the [`CTEST_BUILD_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BUILD_PRESET.html#variable:CTEST_BUILD_PRESET) or [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET) variables.

See also the [`CTEST_PRESETS_FILE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESETS_FILE.html#variable:CTEST_PRESETS_FILE) variable.

**`RETURN_VALUE <result-var>`**

Store the return value of the native build tool in the given variable.

**`CAPTURE_CMAKE_ERROR <result-var>`**

Added in version 3.7.

Store in the `<result-var>` variable -1 if there are any errors running the command and prevent ctest from returning non-zero if an error occurs.

**`QUIET`**

Added in version 3.3.

Suppress any CTest-specific non-error output that would have been printed to the console otherwise. The summary of warnings / errors, as well as the output from the native build tool is unaffected by this option.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
