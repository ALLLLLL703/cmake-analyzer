CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_RUNTIME_CHECKS.html)

# CMAKE_MSVC_RUNTIME_CHECKS

Added in version 4.0.

Select the list of enabled runtime checks when targeting the MSVC ABI. This variable is used to initialize the [`MSVC_RUNTIME_CHECKS`](https://cmake.org/cmake/help/v4.4/prop_tgt/MSVC_RUNTIME_CHECKS.html#prop_tgt:MSVC_RUNTIME_CHECKS) property on all targets as they are created. It is also propagated by calls to the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command into the test project.

The allowed values are:

**`PossibleDataLoss`**

Compile with `-RTCc` or equivalent flag(s) to enable possible data loss checks.

**`StackFrameErrorCheck`**

Compile with `-RTCs` or equivalent flag(s) to enable stack frame error checks.

**`UninitializedVariable`**

Compile with `-RTCu` or equivalent flag(s) to enable uninitialized variables checks.

The value is ignored on compilers not targeting the MSVC ABI, but an unsupported value will be rejected as an error when using a compiler targeting the MSVC ABI.

The value may also be the empty string (`""`), in which case no runtime error check flags will be added explicitly by CMake.

Use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) to support per-configuration specification. For example, the code:

~~~
set(CMAKE_MSVC_RUNTIME_CHECKS "$<$<CONFIG:Debug,RelWithDebInfo>:PossibleDataLoss;UninitializedVariable>")
~~~

enables for the target `foo` the possible data loss and uninitialized variables checks for the `Debug` and `RelWithDebInfo` configurations.

If this variable is not set, the [`MSVC_RUNTIME_CHECKS`](https://cmake.org/cmake/help/v4.4/prop_tgt/MSVC_RUNTIME_CHECKS.html#prop_tgt:MSVC_RUNTIME_CHECKS) target property will not be set automatically. If that property is not set, CMake selects runtime checks using the default value `$<$<CONFIG:Debug>:StackFrameErrorCheck;UninitializedVariable>`, if supported by the compiler, or empty value otherwise.

Note

This variable has effect only when policy [`CMP0184`](https://cmake.org/cmake/help/v4.4/policy/CMP0184.html#policy:CMP0184) is set to `NEW` prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command that enables a language using a compiler targeting the MSVC ABI.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
