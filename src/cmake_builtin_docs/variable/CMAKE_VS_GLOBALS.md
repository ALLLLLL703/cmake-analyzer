CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_GLOBALS.html)

# CMAKE_VS_GLOBALS

Added in version 3.13.

List of `Key=Value` records to be set per target as target properties [`VS_GLOBAL_<variable>`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_GLOBAL_variable.html#prop_tgt:VS_GLOBAL_%3Cvariable%3E) with `variable=Key` and value `Value`.

For example:

~~~
set(CMAKE_VS_GLOBALS
  "DefaultLanguage=en-US"
  "MinimumVisualStudioVersion=14.0"
  )
~~~

will set properties `VS_GLOBAL_DefaultLanguage` to `en-US` and `VS_GLOBAL_MinimumVisualStudioVersion` to `14.0` for all targets (except for `INTERFACE` libraries).

This variable is meant to be set by a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
