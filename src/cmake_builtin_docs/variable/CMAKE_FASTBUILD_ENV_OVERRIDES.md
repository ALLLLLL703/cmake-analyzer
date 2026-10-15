CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FASTBUILD_ENV_OVERRIDES.html)

# CMAKE_FASTBUILD_ENV_OVERRIDES

Added in version 4.2.

Allows overriding environment variables in the captured environment written to `fbuild.bff`.

Specify a CMake-style list of key=value pairs. These values will override the corresponding variables in the environment block that FASTBuild uses during execution of tools (e.g., compilers, linkers, resource compilers, etc.).

This is especially useful for ensuring consistent behavior when tools depend on environment variables (e.g., overriding `PATH` to control tool resolution for `rc.exe` or `mt.exe`).

Example:

~~~
set(CMAKE_FASTBUILD_ENV_OVERRIDES
    "PATH=C:/MyTools/bin"
    "TMP=C:/temp"
    "MY_CUSTOM_VAR=some_value"
)
~~~

Note

This only affects the environment seen by FASTBuild-generated rules. It does **not** modify the environment in which CMake itself runs.

Defaults to empty (no overrides).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
