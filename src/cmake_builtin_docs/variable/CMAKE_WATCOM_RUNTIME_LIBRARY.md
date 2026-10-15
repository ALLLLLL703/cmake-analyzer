CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_WATCOM_RUNTIME_LIBRARY.html)

# CMAKE_WATCOM_RUNTIME_LIBRARY

Added in version 3.24.

Select the Watcom runtime library for use by compilers targeting the Watcom ABI. This variable is used to initialize the [`WATCOM_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/prop_tgt/WATCOM_RUNTIME_LIBRARY.html#prop_tgt:WATCOM_RUNTIME_LIBRARY) property on all targets as they are created. It is also propagated by calls to the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command into the test project.

The allowed values are:

**`SingleThreaded`**

Compile without additional flags to use a single-threaded statically-linked runtime library.

**`SingleThreadedDLL`**

Compile with `-br` or equivalent flag(s) to use a single-threaded dynamically-linked runtime library. This is not available for Linux targets.

**`MultiThreaded`**

Compile with `-bm` or equivalent flag(s) to use a multi-threaded statically-linked runtime library.

**`MultiThreadedDLL`**

Compile with `-bm -br` or equivalent flag(s) to use a multi-threaded dynamically-linked runtime library. This is not available for Linux targets.

The value is ignored on non-Watcom compilers but an unsupported value will be rejected as an error when using a compiler targeting the Watcom ABI.

The value may also be the empty string (`""`) in which case no runtime library selection flag will be added explicitly by CMake.

Use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) to support per-configuration specification.

For example, the code:

~~~
set(CMAKE_WATCOM_RUNTIME_LIBRARY "MultiThreaded")
~~~

selects for all following targets a multi-threaded statically-linked runtime library.

If this variable is not set then the [`WATCOM_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/prop_tgt/WATCOM_RUNTIME_LIBRARY.html#prop_tgt:WATCOM_RUNTIME_LIBRARY) target property will not be set automatically. If that property is not set then CMake uses the default value `MultiThreadedDLL` on Windows and `SingleThreaded` on other platforms to select a Watcom runtime library.

Note

This variable has effect only when policy [`CMP0136`](https://cmake.org/cmake/help/v4.4/policy/CMP0136.html#policy:CMP0136) is set to `NEW` prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command that enables a language using a compiler targeting the Watcom ABI.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
