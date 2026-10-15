CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_LINKER_FLAGS_CONFIG.html)

# CMAKE_MODULE_LINKER_FLAGS_\<CONFIG\>

Flags to be used when linking a module.

These flags will be used by the linker when creating a module for the `<CONFIG>` configuration.

## Handling Compiler Driver Differences

Added in version 4.0.

To pass options to the linker tool, each compiler driver has its own syntax. The `LINKER:` prefix and `,` separator can be used to specify, in a portable way, options to pass to the linker tool. `LINKER:` is replaced by the appropriate driver option and `,` by the appropriate driver separator. The driver prefix and driver separator are given by the values of the [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG) and [`CMAKE_<LANG>_LINKER_WRAPPER_FLAG_SEP`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_WRAPPER_FLAG_SEP.html#variable:CMAKE_%3CLANG%3E_LINKER_WRAPPER_FLAG_SEP) variables.

For example, `"LINKER:-z,defs"` becomes `-Xlinker -z -Xlinker defs` for `Clang` and `-Wl,-z,defs` for `GNU GCC`.

The `LINKER:` prefix supports, as an alternative syntax, specification of arguments using the `SHELL:` prefix and space as separator. The previous example then becomes `"LINKER:SHELL:-z defs"`.

Note

Specifying the `SHELL:` prefix anywhere other than at the beginning of the `LINKER:` prefix is not supported.

This support implies to parse and re-quote the content of the variable. See policy [`CMP0181`](https://cmake.org/cmake/help/v4.4/policy/CMP0181.html#policy:CMP0181).

## See Also

- [`CMAKE_MODULE_LINKER_FLAGS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_LINKER_FLAGS.html#variable:CMAKE_MODULE_LINKER_FLAGS)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
