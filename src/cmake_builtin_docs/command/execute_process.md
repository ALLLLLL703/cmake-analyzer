CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/execute_process.html)

# execute_process

Execute one or more child processes.

~~~
execute_process(COMMAND <cmd1> [<arguments>]
                [COMMAND <cmd2> [<arguments>]]...
                [WORKING_DIRECTORY <directory>]
                [TIMEOUT <seconds>]
                [RESULT_VARIABLE <variable>]
                [RESULTS_VARIABLE <variable>]
                [OUTPUT_VARIABLE <variable>]
                [ERROR_VARIABLE <variable>]
                [INPUT_FILE <file>]
                [OUTPUT_FILE <file>]
                [ERROR_FILE <file>]
                [OUTPUT_QUIET]
                [ERROR_QUIET]
                [COMMAND_ECHO <where>]
                [OUTPUT_STRIP_TRAILING_WHITESPACE]
                [ERROR_STRIP_TRAILING_WHITESPACE]
                [ENCODING <name>]
                [ENVIRONMENT <variable=value>...]
                [ENVIRONMENT_MODIFICATION <env-mod-op>...]
                [ECHO_OUTPUT_VARIABLE]
                [ECHO_ERROR_VARIABLE]
                [COMMAND_ERROR_IS_FATAL <ANY|LAST|NONE>])
~~~

Runs the given sequence of one or more commands.

Commands are executed concurrently as a pipeline, with the standard output of each process piped to the standard input of the next. A single standard error pipe is used for all processes.

`execute_process` runs commands while CMake is configuring the project, prior to build system generation. Use the [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) and [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) commands to create custom commands that run at build time.

Options:

**`COMMAND`**

A child process command line.

CMake executes the child process using operating system APIs directly:

- On POSIX platforms, the command line is passed to the child process in an `argv[]` style array. No intermediate shell is executed, so shell operators such as `>` are treated as normal arguments.

- On Windows platforms, the command line is encoded as a string such that child processes using [CommandLineToArgvW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw) will decode the original arguments.

   

  If the command runs a `.exe`, `.com`, or other executable, no intermediate command interpreter is executed, so shell operators such as `>` are treated as normal arguments.

   

  If the command runs a `.bat` or `.cmd` script, it is executed through the `cmd` command interpreter. The command interpreter does not use [CommandLineToArgvW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw), so some arguments may be received by the script with extra quoting.

    

  Changed in version 4.0: `.bat` and `.cmd` scripts are now explicitly executed through the command interpreter by prepending `cmd /c call` to the command line. Previously, they were implicitly executed through `cmd /c`, without `call`, by undocumented behavior of [CreateProcessW](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw).

Use the `INPUT_*`, `OUTPUT_*`, and `ERROR_*` options to redirect stdin, stdout, and stderr.

For **sequential execution** of multiple commands use multiple `execute_process` calls each with a single `COMMAND` argument.

**`WORKING_DIRECTORY`**

The named directory will be set as the current working directory of the child processes. If not given, the child processes' working directory is unspecified.

**`TIMEOUT`**

After the specified number of seconds (fractions allowed), all unfinished child processes will be terminated, and the `RESULT_VARIABLE` will be set to a string mentioning the "timeout".

**`RESULT_VARIABLE`**

The variable will be set to contain the result of last child process. This will be an integer return code from the last child or a string describing an error condition.

**`RESULTS_VARIABLE <variable>`**

Added in version 3.10.

The variable will be set to contain the result of all processes as a [semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists), in order of the given `COMMAND` arguments. Each entry will be an integer return code from the corresponding child or a string describing an error condition.

**`INPUT_FILE <file>`**

`<file>` is attached to the standard input pipe of the *first* `COMMAND` process.

**`OUTPUT_FILE <file>`**

`<file>` is attached to the standard output pipe of the *last* `COMMAND` process.

**`ERROR_FILE <file>`**

`<file>` is attached to the standard error pipe of *all* `COMMAND` processes.

Added in version 3.3: If the same `<file>` is named for both `OUTPUT_FILE` and `ERROR_FILE` then it will be used for both standard output and standard error pipes.

**`OUTPUT_QUIET`, `ERROR_QUIET`**

The standard output on `OUTPUT_VARIABLE` or standard error on `ERROR_VARIABLE` are not connected (no variable content). The `*_FILE` and `ECHO_*_VARIABLE` options are not affected.

**`OUTPUT_VARIABLE`, `ERROR_VARIABLE`**

The variable named will be set with the contents of the standard output and standard error pipes, respectively. If the same variable is named for both pipes their output will be merged in the order produced.

**`OUTPUT_STRIP_TRAILING_WHITESPACE`, `ERROR_STRIP_TRAILING_WHITESPACE`**

Removes whitespace characters from the end of the values stored in the corresponding `OUTPUT_VARIABLE` and `ERROR_VARIABLE` variables.

**`ECHO_OUTPUT_VARIABLE`, `ECHO_ERROR_VARIABLE`**

Added in version 3.18.

The standard output or standard error will not be exclusively redirected to the specified variables.

The output will be duplicated into the specified variables and also onto standard output or standard error analogous to the `tee` Unix command.

Note

If more than one `OUTPUT_*` or `ERROR_*` option is given for the same pipe the precedence is *not specified*. If no `OUTPUT_*` or `ERROR_*` options are given the output will be shared with the corresponding pipes of the CMake process itself.

**`COMMAND_ECHO <where>`**

Added in version 3.15.

The command being run will be echo'ed to `<where>` with `<where>` being set to one of `STDERR`, `STDOUT` or `NONE`. See the [`CMAKE_EXECUTE_PROCESS_COMMAND_ECHO`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXECUTE_PROCESS_COMMAND_ECHO.html#variable:CMAKE_EXECUTE_PROCESS_COMMAND_ECHO) variable for a way to control the default behavior when this option is not present.

**`ENCODING <name>`**

Added in version 3.8.

On Windows, the encoding that is used to decode output from the process. Ignored on other platforms. Valid encoding names are:

**`NONE`**

Perform no decoding. This assumes that the process output is encoded in the same way as CMake's internal encoding (UTF-8).

This was the default in CMake 3.14 and older.

**`AUTO`**

Use the current active console's codepage or if that isn't available then use ANSI.

This was the default in CMake 3.15 through 3.30.

**`ANSI`**

Use the ANSI codepage.

**`OEM`**

Use the original equipment manufacturer (OEM) code page.

**`UTF-8`**

Added in version 3.11.

Use the UTF-8 codepage.

This is the default since CMake 3.31. See policy [`CMP0176`](https://cmake.org/cmake/help/v4.4/policy/CMP0176.html#policy:CMP0176).

**`UTF8`**

Use the UTF-8 codepage. Use of this name is discouraged in favor of `UTF-8` to match the [UTF-8 RFC](https://datatracker.ietf.org/doc/html/rfc3629) naming convention.

**`ENVIRONMENT <variable=value>...`**

Added in version 4.4.

Add environment variable definitions to the environment of the child processes. Each entry must be of the form `NAME=value` and is interpreted as setting `NAME` to `value` for the duration of the `execute_process()` call.

These entries are processed in order. If the same variable name is given multiple times, the last value wins.

**`ENVIRONMENT_MODIFICATION <env-mod-op>...`**

Added in version 4.4.

Apply one or more [`ENVIRONMENT_MODIFICATION`](https://cmake.org/cmake/help/v4.4/prop_test/ENVIRONMENT_MODIFICATION.html#prop_test:ENVIRONMENT_MODIFICATION) operations to the environment of the child processes. Each operation is applied in the given order, and failures to parse an operation are treated as errors.

Environment modifications are applied after `ENVIRONMENT` entries, if both options are specified.

**`COMMAND_ERROR_IS_FATAL <ANY|LAST|NONE>`**

Added in version 3.19.

The option following `COMMAND_ERROR_IS_FATAL` determines the behavior when an error is encountered:

**`ANY`**

If any of the commands in the list of commands fail, the `execute_process()` command halts with an error.

**`LAST`**

If the last command in the list of commands fails, the `execute_process()` command halts with an error. Commands earlier in the list will not cause a fatal error.

**`NONE`**

Added in version 4.0.

Regardless of any of the commands failing, the `execute_process()` command will not halt with an error.

Added in version 4.0: If not provided, the [`CMAKE_EXECUTE_PROCESS_COMMAND_ERROR_IS_FATAL`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXECUTE_PROCESS_COMMAND_ERROR_IS_FATAL.html#variable:CMAKE_EXECUTE_PROCESS_COMMAND_ERROR_IS_FATAL) variable is checked. If the variable is not set, the default is `NONE`. If `RESULT_VARIABLE` or `RESULTS_VARIABLE` is supplied, [`CMAKE_EXECUTE_PROCESS_COMMAND_ERROR_IS_FATAL`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXECUTE_PROCESS_COMMAND_ERROR_IS_FATAL.html#variable:CMAKE_EXECUTE_PROCESS_COMMAND_ERROR_IS_FATAL) is ignored.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
