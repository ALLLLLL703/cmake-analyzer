CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_POSITION_INDEPENDENT_CODE.html)

# CMAKE_POSITION_INDEPENDENT_CODE

Default value for the [`POSITION_INDEPENDENT_CODE`](https://cmake.org/cmake/help/v4.4/prop_tgt/POSITION_INDEPENDENT_CODE.html#prop_tgt:POSITION_INDEPENDENT_CODE) target property.

This variable is used to initialize the [`POSITION_INDEPENDENT_CODE`](https://cmake.org/cmake/help/v4.4/prop_tgt/POSITION_INDEPENDENT_CODE.html#prop_tgt:POSITION_INDEPENDENT_CODE) property on targets that are not `SHARED` or `MODULE` library targets. If set, its value is also used by the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command.

The `SHARED` and `MODULE` library targets have by default position independent code enabled regardless of this variable. To disable PIC on these library types, only manually setting the target property disables it.

## See Also

- The [`CheckPIESupported`](https://cmake.org/cmake/help/v4.4/module/CheckPIESupported.html#module:CheckPIESupported) module to pass PIE-related options to the linker for executables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
