CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_DEBUG_INFORMATION_FORMAT.html)

# CMAKE_MSVC_DEBUG_INFORMATION_FORMAT

Added in version 3.25.

Select the MSVC debug information format targeting the MSVC ABI. This variable is used to initialize the [`MSVC_DEBUG_INFORMATION_FORMAT`](https://cmake.org/cmake/help/v4.4/prop_tgt/MSVC_DEBUG_INFORMATION_FORMAT.html#prop_tgt:MSVC_DEBUG_INFORMATION_FORMAT) property on all targets as they are created. It is also propagated by calls to the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command into the test project.

The allowed values are:

**`Embedded`**

Compile with `-Z7` or equivalent flag(s) to produce object files with full symbolic debugging information.

**`ProgramDatabase`**

Compile with `-Zi` or equivalent flag(s) to produce a program database that contains all the symbolic debugging information.

**`EditAndContinue`**

Compile with `-ZI` or equivalent flag(s) to produce a program database that supports the Edit and Continue feature.

The value is ignored on compilers not targeting the MSVC ABI, but an unsupported value will be rejected as an error when using a compiler targeting the MSVC ABI.

The value may also be the empty string (`""`), in which case no debug information format flag will be added explicitly by CMake.

Use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) to support per-configuration specification. For example, the code:

~~~
set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>")
~~~

selects for all following targets the program database debug information format for the `Debug` and `RelWithDebInfo` configurations.

If this variable is not set, the [`MSVC_DEBUG_INFORMATION_FORMAT`](https://cmake.org/cmake/help/v4.4/prop_tgt/MSVC_DEBUG_INFORMATION_FORMAT.html#prop_tgt:MSVC_DEBUG_INFORMATION_FORMAT) target property will not be set automatically. If that property is not set, CMake selects a debug information format using the default value `$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>`, if supported by the compiler, and otherwise `$<$<CONFIG:Debug,RelWithDebInfo>:Embedded>`.

Note

This variable has effect only when policy [`CMP0141`](https://cmake.org/cmake/help/v4.4/policy/CMP0141.html#policy:CMP0141) is set to `NEW` prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command that enables a language using a compiler targeting the MSVC ABI.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
