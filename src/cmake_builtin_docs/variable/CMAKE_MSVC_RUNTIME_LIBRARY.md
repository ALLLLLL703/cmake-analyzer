CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_RUNTIME_LIBRARY.html)

# CMAKE_MSVC_RUNTIME_LIBRARY

Added in version 3.15.

Select the MSVC runtime library for use by compilers targeting the MSVC ABI. This variable is used to initialize the [`MSVC_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/prop_tgt/MSVC_RUNTIME_LIBRARY.html#prop_tgt:MSVC_RUNTIME_LIBRARY) property on all targets as they are created. It is also propagated by calls to the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command into the test project.

The allowed values are:

**`""` (set but empty)**

No runtime library selection flag will be added explicitly by CMake.

Note that with [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) the native build system may choose to add its own default runtime library selection flag.

**`MultiThreaded`**

Compile with `-MT` or equivalent flag(s) to use a multi-threaded statically-linked runtime library.

**`MultiThreadedDLL`**

Compile with `-MD` or equivalent flag(s) to use a multi-threaded dynamically-linked runtime library.

**`MultiThreadedDebug`**

Compile with `-MTd` or equivalent flag(s) to use a multi-threaded statically-linked runtime library.

**`MultiThreadedDebugDLL`**

Compile with `-MDd` or equivalent flag(s) to use a multi-threaded dynamically-linked runtime library.

The value is ignored on compilers not targeting the MSVC ABI, but an unsupported value will be rejected as an error when using a compiler targeting the MSVC ABI.

Use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) to support per-configuration specification. For example, the code:

~~~
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
~~~

selects for all following targets a multi-threaded statically-linked runtime library with or without debug information depending on the configuration.

If this variable is not set then the [`MSVC_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/prop_tgt/MSVC_RUNTIME_LIBRARY.html#prop_tgt:MSVC_RUNTIME_LIBRARY) target property will not be set automatically. If that property is not set then CMake uses the default value `MultiThreaded$<$<CONFIG:Debug>:Debug>DLL` to select a MSVC runtime library.

Note

This variable has effect only when policy [`CMP0091`](https://cmake.org/cmake/help/v4.4/policy/CMP0091.html#policy:CMP0091) is set to `NEW` prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command that enables a language using a compiler targeting the MSVC ABI.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
