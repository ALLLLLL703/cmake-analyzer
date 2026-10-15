CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ADD_CUSTOM_COMMAND_DEPENDS_EXPLICIT_ONLY.html)

# CMAKE_ADD_CUSTOM_COMMAND_DEPENDS_EXPLICIT_ONLY

Added in version 3.27.

Whether to enable the `DEPENDS_EXPLICIT_ONLY` option by default in [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command).

This variable affects the default behavior of the [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) command. Setting this variable to `ON` is equivalent to using the `DEPENDS_EXPLICIT_ONLY` option in all uses of that command.

See also [`CMAKE_OPTIMIZE_DEPENDENCIES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OPTIMIZE_DEPENDENCIES.html#variable:CMAKE_OPTIMIZE_DEPENDENCIES).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
