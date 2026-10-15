CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS.html)

# CMAKE_\<LANG\>_LINK_FLAGS

Added in version 4.3.

Language-wide flags for language `<LANG>` used when linking for all configurations. These flags will be passed to all invocations of the compiler which drive linking.

The flags in this variable will obey the following behavior with respect to ordering of flags from other variables.

- They will be passed after those added by [`CMAKE_<LANG>_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html#variable:CMAKE_%3CLANG%3E_FLAGS) and [`CMAKE_<LANG>_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_FLAGS_%3CCONFIG%3E).

- They will be passed after those added by [`CMAKE_EXE_LINKER_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXE_LINKER_FLAGS.html#variable:CMAKE_EXE_LINKER_FLAGS), [`CMAKE_EXE_LINKER_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXE_LINKER_FLAGS_CONFIG.html#variable:CMAKE_EXE_LINKER_FLAGS_%3CCONFIG%3E), [`CMAKE_SHARED_LINKER_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LINKER_FLAGS.html#variable:CMAKE_SHARED_LINKER_FLAGS), [`CMAKE_SHARED_LINKER_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LINKER_FLAGS_CONFIG.html#variable:CMAKE_SHARED_LINKER_FLAGS_%3CCONFIG%3E), [`CMAKE_MODULE_LINKER_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_LINKER_FLAGS.html#variable:CMAKE_MODULE_LINKER_FLAGS), and [`CMAKE_MODULE_LINKER_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_LINKER_FLAGS_CONFIG.html#variable:CMAKE_MODULE_LINKER_FLAGS_%3CCONFIG%3E) depending on the given target type.

- They will be passed before those added by [`CMAKE_<LANG>_LINK_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS_%3CCONFIG%3E).

- They will be passed before those added by commands such as [`add_link_options()`](https://cmake.org/cmake/help/v4.4/command/add_link_options.html#command:add_link_options) and [`target_link_options()`](https://cmake.org/cmake/help/v4.4/command/target_link_options.html#command:target_link_options).

Use of this variable is enabled when policy [`CMP0210`](https://cmake.org/cmake/help/v4.4/policy/CMP0210.html#policy:CMP0210) is `NEW`.

## Handling Compiler Driver Differences

Added in version 4.0.

To pass options to the linker tool, each compiler driver has its own syntax. The `LINKER:` prefix and `,` separator can be used to specify, in a portable way, options to pass to the linker tool. `LINKER:` is replaced by the appropriate driver option and `,` by the appropriate driver separator. The driver prefix and driver separator are given by the values of the [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) and [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG_SEP) variables.

For example, `"LINKER:-z,defs"` becomes `-Xlinker -z -Xlinker defs` for `Clang` and `-Wl,-z,defs` for `GNU GCC`.

The `LINKER:` prefix supports, as an alternative syntax, specification of arguments using the `SHELL:` prefix and space as separator. The previous example then becomes `"LINKER:SHELL:-z defs"`.

Note

Specifying the `SHELL:` prefix anywhere other than at the beginning of the `LINKER:` prefix is not supported.

This support implies to parse and re-quote the content of the variable.

## See Also

- [`CMAKE_<LANG>_LINK_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_FLAGS_CONFIG.html#variable:CMAKE_%3CLANG%3E_LINK_FLAGS_%3CCONFIG%3E)

- [`CMAKE_<LANG>_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_FLAGS.html#variable:CMAKE_%3CLANG%3E_FLAGS)

- [`CMP0210`](https://cmake.org/cmake/help/v4.4/policy/CMP0210.html#policy:CMP0210)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
