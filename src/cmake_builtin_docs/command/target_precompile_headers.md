CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html)

# target_precompile_headers

Added in version 3.16.

Add a list of header files to precompile.

Precompiling header files can speed up compilation by creating a partially processed version of some header files, and then using that version during compilations rather than repeatedly parsing the original headers.

## Main Form

~~~
target_precompile_headers(<target>
  {INTERFACE|PUBLIC|PRIVATE} <header>...
  [{INTERFACE|PUBLIC|PRIVATE} <header>...]...)
~~~

The command adds header files to the [`PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRECOMPILE_HEADERS.html#prop_tgt:PRECOMPILE_HEADERS) and/or [`INTERFACE_PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_PRECOMPILE_HEADERS.html#prop_tgt:INTERFACE_PRECOMPILE_HEADERS) target properties of `<target>`. The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the [scope](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#target-command-scope) of the following arguments. `PRIVATE` and `PUBLIC` items will populate the [`PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRECOMPILE_HEADERS.html#prop_tgt:PRECOMPILE_HEADERS) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_PRECOMPILE_HEADERS.html#prop_tgt:INTERFACE_PRECOMPILE_HEADERS) property of `<target>` ([IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets) only support `INTERFACE` items). Repeated calls for the same `<target>` will append items in the order called.

Projects should generally avoid using `PUBLIC` or `INTERFACE` for targets that will be [`exported`](https://cmake.org/cmake/help/v4.4/command/install.html#export), or they should at least use the [`$<BUILD_INTERFACE:...>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:BUILD_INTERFACE) generator expression to prevent precompile headers from appearing in an installed exported target. Consumers of a target should typically be in control of what precompile headers they use, not have precompile headers forced on them by the targets being consumed (since precompile headers are not typically usage requirements). A notable exception to this is where an [interface library](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#interface-libraries) is created to define a commonly used set of precompile headers in one place and then other targets link to that interface library privately. In this case, the interface library exists specifically to propagate the precompile headers to its consumers and the consumer is effectively still in control, since it decides whether to link to the interface library or not.

The list of header files is used to generate a header file named `cmake_pch.h|xx` which is used to generate the precompiled header file (`.pch`, `.gch`, `.pchi`) artifact. The `cmake_pch.h|xx` header file will be force included (`-include` for GCC, `/FI` for MSVC) to all source files, so sources do not need to have `#include "pch.h"`.

Header file names specified with angle brackets (e.g. `<unordered_map>`) or explicit double quotes (escaped for the [`cmake-language(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#manual:cmake-language(7)), e.g. `[["other_header.h"]]`) will be treated as is, and include directories must be available for the compiler to find them. Other header file names (e.g. `project_header.h`) are interpreted as being relative to the current source directory (e.g. [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR)) and will be included by absolute path. For example:

~~~
target_precompile_headers(myTarget
  PUBLIC
    project_header.h
  PRIVATE
    [["other_header.h"]]
    <unordered_map>
)
~~~

Arguments to `target_precompile_headers` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. The [`$<COMPILE_LANGUAGE:...>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:COMPILE_LANGUAGE:languages) generator expression is particularly useful for specifying a language-specific header to precompile for only one language (e.g. `CXX` and not `C`). In this case, header file names that are not explicitly in double quotes or angle brackets must be specified by absolute path. Also, when specifying angle brackets inside a generator expression, be sure to encode the closing `>` as [`$<ANGLE-R>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:ANGLE-R). For example:

~~~
target_precompile_headers(mylib PRIVATE
  "$<$<COMPILE_LANGUAGE:CXX>:${CMAKE_CURRENT_SOURCE_DIR}/cxx_only.h>"
  "$<$<COMPILE_LANGUAGE:C>:<stddef.h$<ANGLE-R>>"
  "$<$<COMPILE_LANGUAGE:CXX>:<cstddef$<ANGLE-R>>"
)
~~~

## Reusing Precompile Headers

The command also supports a second signature which can be used to specify that one target reuses a precompiled header file artifact from another target instead of generating its own:

~~~
target_precompile_headers(<target> REUSE_FROM <other_target>)
~~~

This form sets the [`PRECOMPILE_HEADERS_REUSE_FROM`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRECOMPILE_HEADERS_REUSE_FROM.html#prop_tgt:PRECOMPILE_HEADERS_REUSE_FROM) property to `<other_target>` and adds a dependency such that `<target>` will depend on `<other_target>`. CMake will halt with an error if the [`PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/PRECOMPILE_HEADERS.html#prop_tgt:PRECOMPILE_HEADERS) property of `<target>` is already set when the `REUSE_FROM` form is used.

Note

The `REUSE_FROM` form requires the same set of compiler options, compiler flags and compiler definitions for both `<target>` and `<other_target>`. Some compilers (e.g. GCC) may issue a warning if the precompiled header file cannot be used (`-Winvalid-pch`).

## See Also

- To disable precompile headers for specific targets, see the [`DISABLE_PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_tgt/DISABLE_PRECOMPILE_HEADERS.html#prop_tgt:DISABLE_PRECOMPILE_HEADERS) target property.

- To prevent precompile headers from being used when compiling a specific source file, see the [`SKIP_PRECOMPILE_HEADERS`](https://cmake.org/cmake/help/v4.4/prop_sf/SKIP_PRECOMPILE_HEADERS.html#prop_sf:SKIP_PRECOMPILE_HEADERS) source file property.

- [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions)

- [`target_compile_features()`](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html#command:target_compile_features)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
