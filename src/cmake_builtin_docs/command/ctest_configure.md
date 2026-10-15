CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/ctest_configure.html)

# ctest_configure

Perform the [CTest Configure Step](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#ctest-configure-step) as a [Dashboard Client](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#dashboard-client).

~~~
ctest_configure([BUILD <build-dir>] [SOURCE <source-dir>] [APPEND]
                [OPTIONS <options>] [RETURN_VALUE <result-var>] [QUIET]
                [CAPTURE_CMAKE_ERROR <result-var>]
                [PRESET <preset>]
                [PRESETS_FILE <file>]
                )
~~~

Configure the project build tree and record results in `Configure.xml` for submission with the [`ctest_submit()`](https://cmake.org/cmake/help/v4.4/command/ctest_submit.html#command:ctest_submit) command.

The options are:

**`BUILD <build-dir>`**

Specify the top-level build directory. If not given, the [`CTEST_BINARY_DIRECTORY`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BINARY_DIRECTORY.html#variable:CTEST_BINARY_DIRECTORY) variable is used.

**`SOURCE <source-dir>`**

Specify the source directory. If not given, the [`CTEST_SOURCE_DIRECTORY`](https://cmake.org/cmake/help/v4.4/variable/CTEST_SOURCE_DIRECTORY.html#variable:CTEST_SOURCE_DIRECTORY) variable is used.

**`APPEND`**

Mark `Configure.xml` for append to results previously submitted to a dashboard server since the last [`ctest_start()`](https://cmake.org/cmake/help/v4.4/command/ctest_start.html#command:ctest_start) call. Append semantics are defined by the dashboard server in use. This does *not* cause results to be appended to a `.xml` file produced by a previous call to this command.

**`OPTIONS <options>`**

Specify a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of command-line arguments to pass to the configuration tool. This option is ignored when [`CTEST_CONFIGURE_COMMAND`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CONFIGURE_COMMAND.html#variable:CTEST_CONFIGURE_COMMAND) is used.

**`PRESET <preset>`**

Added in version 4.4.

Specify a [`preset`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#manual:cmake-presets(7)) to use when configuring the project. Any value set in the CTest script will take priority over a corresponding setting from the preset. For example, the [`CTEST_BINARY_DIRECTORY`](https://cmake.org/cmake/help/v4.4/variable/CTEST_BINARY_DIRECTORY.html#variable:CTEST_BINARY_DIRECTORY) variable will override the [`binaryDir`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#cmakepresets-configurepresets-binarydir) setting from the chosen preset.

When a preset is specified, the [`CTEST_CONFIGURE_COMMAND`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CONFIGURE_COMMAND.html#variable:CTEST_CONFIGURE_COMMAND) variable is ignored so that the preset is always honored.

See also the [`CTEST_CONFIGURE_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CONFIGURE_PRESET.html#variable:CTEST_CONFIGURE_PRESET) and [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET) variables.

**`PRESETS_FILE <file>`**

Added in version 4.4.

Specify a [`presets`](https://cmake.org/cmake/help/v4.4/manual/cmake-presets.7.html#manual:cmake-presets(7)) file to use instead of the default `CMakePresets.json` in the source directory. A relative path is interpreted relative to the source directory. Has no effect unless a preset is selected via the `PRESET` argument or the [`CTEST_CONFIGURE_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_CONFIGURE_PRESET.html#variable:CTEST_CONFIGURE_PRESET) or [`CTEST_PRESET`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESET.html#variable:CTEST_PRESET) variables.

See also the [`CTEST_PRESETS_FILE`](https://cmake.org/cmake/help/v4.4/variable/CTEST_PRESETS_FILE.html#variable:CTEST_PRESETS_FILE) variable.

**`RETURN_VALUE <result-var>`**

Store in the `<result-var>` variable the return value of the native configuration tool.

**`CAPTURE_CMAKE_ERROR <result-var>`**

Added in version 3.7.

Store in the `<result-var>` variable `-1` if there are any errors running the command and prevent [`ctest(1)`](https://cmake.org/cmake/help/v4.4/manual/ctest.1.html#manual:ctest(1)) from returning non-zero if an error occurs.

**`QUIET`**

Added in version 3.3.

Suppress any CTest-specific non-error messages that would have otherwise been printed to the console. Output from the underlying configure command is not affected.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
