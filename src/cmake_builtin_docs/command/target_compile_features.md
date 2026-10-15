CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/target_compile_features.html)

# target_compile_features

Added in version 3.1.

Add expected compiler features to a target.

~~~
target_compile_features(<target>
  {INTERFACE|PUBLIC|PRIVATE} <feature>...
  [{INTERFACE|PUBLIC|PRIVATE} <feature>...]...)
~~~

Specifies compiler features required when compiling a given target. If the feature is not listed in the [`CMAKE_C_COMPILE_FEATURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_C_COMPILE_FEATURES.html#variable:CMAKE_C_COMPILE_FEATURES), [`CMAKE_CUDA_COMPILE_FEATURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CUDA_COMPILE_FEATURES.html#variable:CMAKE_CUDA_COMPILE_FEATURES), or [`CMAKE_CXX_COMPILE_FEATURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CXX_COMPILE_FEATURES.html#variable:CMAKE_CXX_COMPILE_FEATURES) variables, then an error will be reported by CMake. If the use of the feature requires an additional compiler flag, such as `-std=gnu++11`, the flag will be added automatically.

The `INTERFACE`, `PUBLIC` and `PRIVATE` keywords are required to specify the scope of the features. `PRIVATE` and `PUBLIC` items will populate the [`COMPILE_FEATURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/COMPILE_FEATURES.html#prop_tgt:COMPILE_FEATURES) property of `<target>`. `PUBLIC` and `INTERFACE` items will populate the [`INTERFACE_COMPILE_FEATURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_COMPILE_FEATURES.html#prop_tgt:INTERFACE_COMPILE_FEATURES) property of `<target>`. Repeated calls for the same `<target>` append items.

Added in version 3.11: Allow setting `INTERFACE` items on [IMPORTED targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#imported-targets).

The named `<target>` must have been created by a command such as [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) or [`add_library()`](https://cmake.org/cmake/help/v4.4/command/add_library.html#command:add_library) and must not be an [ALIAS target](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets).

Arguments to `target_compile_features` may use generator expressions with the syntax `$<...>`. See the [`cmake-generator-expressions(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) manual for available expressions. See the [`cmake-compile-features(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-compile-features.7.html#manual:cmake-compile-features(7)) manual for information on compile features and a list of supported compilers.

## See Also

- [`target_compile_definitions()`](https://cmake.org/cmake/help/v4.4/command/target_compile_definitions.html#command:target_compile_definitions)

- [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options)

- [`target_include_directories()`](https://cmake.org/cmake/help/v4.4/command/target_include_directories.html#command:target_include_directories)

- [`target_link_libraries()`](https://cmake.org/cmake/help/v4.4/command/target_link_libraries.html#command:target_link_libraries)

- [`target_link_directories()`](https://cmake.org/cmake/help/v4.4/command/target_link_directories.html#command:target_link_directories)

- [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options)

- [`target_precompile_headers()`](https://cmake.org/cmake/help/v4.4/command/target_precompile_headers.html#command:target_precompile_headers)

- [`target_sources()`](https://cmake.org/cmake/help/v4.4/command/target_sources.html#command:target_sources)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
