CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/get_filename_component.html)

# get_filename_component

Get a specific component of a full filename.

Changed in version 3.20: This command has been superseded by the [`cmake_path()`](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#command:cmake_path) command, except for `REALPATH`, which is now offered by [`file(REAL_PATH)`](https://cmake.org/cmake/help/v4.4/command/file.html#real-path), and `PROGRAM`, now available in [`separate_arguments(PROGRAM)`](https://cmake.org/cmake/help/v4.4/command/separate_arguments.html#command:separate_arguments).

Changed in version 3.24: The undocumented feature offering the capability to query the `Windows` registry is superseded by the [`cmake_host_system_information(QUERY WINDOWS_REGISTRY)`](https://cmake.org/cmake/help/v4.4/command/cmake_host_system_information.html#query-windows-registry) command.

~~~
get_filename_component(<var> <FileName> <mode> [CACHE])
~~~

Sets `<var>` to a component of `<FileName>`, where `<mode>` is one of:

- `DIRECTORY` - directory without file name.

- `NAME` - file name without directory.

- `EXT` - file name longest extension (`.b.c` from `d/a.b.c`).

- `NAME_WE` - file name with neither the directory nor the longest extension.

- `LAST_EXT` - file name last extension (`.c` from `d/a.b.c`).

- `NAME_WLE` - file name with neither the directory nor the last extension.

- `PATH` - legacy alias for `DIRECTORY` (use for CMake \<= 2.8.11).

Added in version 3.14: Added the `LAST_EXT` and `NAME_WLE` modes.

Paths are returned with forward slashes and have no trailing slashes. If the optional `CACHE` argument is specified, the result variable is added to the cache.

~~~
get_filename_component(<var> <FileName> <mode> [BASE_DIR <dir>] [CACHE])
~~~

Added in version 3.4.

Sets `<var>` to the absolute path of `<FileName>`, where `<mode>` is one of:

- `ABSOLUTE` - full path to file.

- `REALPATH` - full path to existing file with symlinks resolved.

If the provided `<FileName>` is a relative path, it is evaluated relative to the given base directory `<dir>`. If no base directory is provided, the default base directory will be [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR).

Paths are returned with forward slashes and have no trailing slashes. If the optional `CACHE` argument is specified, the result variable is added to the cache.

~~~
get_filename_component(<var> <FileName> PROGRAM [PROGRAM_ARGS <arg_var>] [CACHE])
~~~

The program in `<FileName>` will be found in the system search path or left as a full path. If `PROGRAM_ARGS` is present with `PROGRAM`, then any command-line arguments present in the `<FileName>` string are split from the program name and stored in `<arg_var>`. This is used to separate a program name from its arguments in a command line string.

## See Also

- [`cmake_path()`](https://cmake.org/cmake/help/v4.4/command/cmake_path.html#command:cmake_path)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
