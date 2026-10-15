CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_USE_DEBUG_LIBRARIES.html)

# CMAKE_VS_USE_DEBUG_LIBRARIES

Added in version 3.30.

Indicate to [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) what configurations are considered debug configurations. This controls the `UseDebugLibraries` setting in each configuration of a `.vcxproj` file.

The "Use Debug Libraries" setting in Visual Studio projects, despite its specific-sounding name, is a general-purpose indicator of what configurations are considered debug configurations. In standalone projects, this may affect MSBuild's default selection of MSVC runtime library, optimization flags, runtime checks, and similar settings. In CMake projects those settings are typically generated explicitly based on the project's specification, e.g., the MSVC runtime library is controlled by [`CMAKE_MSVC_RUNTIME_LIBRARY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVC_RUNTIME_LIBRARY.html#variable:CMAKE_MSVC_RUNTIME_LIBRARY). However, the `UseDebugLibraries` indicator is useful for reference by both humans and tools, and may also affect the behavior of platform-specific SDKs.

Set `CMAKE_VS_USE_DEBUG_LIBRARIES` to a true or false value to indicate whether each configuration is considered a debug configuration. The value may also be the empty string (`""`) in which case no `UseDebugLibraries` will be added explicitly by CMake, and MSBuild will use its default value, `false`.

Use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) for per-configuration specification. For example, the code:

~~~
set(CMAKE_VS_USE_DEBUG_LIBRARIES "$<CONFIG:Debug,Custom>")
~~~

indicates that all following targets consider their "Debug" and "Custom" configurations to be debug configurations, and their other configurations to be non-debug configurations.

This variable is used to initialize the [`VS_USE_DEBUG_LIBRARIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_USE_DEBUG_LIBRARIES.html#prop_tgt:VS_USE_DEBUG_LIBRARIES) property on all targets as they are created. It is also propagated by calls to the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command into its test project.

If this variable is not set then the [`VS_USE_DEBUG_LIBRARIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_USE_DEBUG_LIBRARIES.html#prop_tgt:VS_USE_DEBUG_LIBRARIES) property will not be set automatically. If that property is not set then CMake generates `UseDebugLibraries` using heuristics to determine which configurations are debug configurations. See policy [`CMP0162`](https://cmake.org/cmake/help/v4.4/policy/CMP0162.html#policy:CMP0162).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
