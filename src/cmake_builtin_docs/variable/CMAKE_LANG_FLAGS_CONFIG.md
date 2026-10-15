CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG.html)

# CMAKE_\<LANG\>_FLAGS_\<CONFIG\>

Language-wide flags for language `<LANG>` used when building for the `<CONFIG>` configuration. These flags will be passed to all invocations of the compiler in the corresponding configuration. This includes invocations that drive compiling and those that drive linking.

The flags in this variable will be passed after those in the [`CMAKE_<LANG>_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html#variable:CMAKE_%3CLANG%3E_FLAGS) variable. On invocations driving compiling, flags from both variables will be passed before flags added by commands such as [`add_compile_options()`](https://cmake.org/cmake/help/v4.4/command/add_compile_options.html#command:add_compile_options) and [`target_compile_options()`](https://cmake.org/cmake/help/v4.4/command/target_compile_options.html#command:target_compile_options). On invocations driving linking, they will be passed before flags added by commands such as [`add_link_options()`](https://cmake.org/cmake/help/v4.4/command/add_link_options.html#command:add_link_options) and [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
