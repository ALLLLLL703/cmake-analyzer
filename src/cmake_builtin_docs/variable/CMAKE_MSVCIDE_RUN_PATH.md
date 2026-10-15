CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MSVCIDE_RUN_PATH.html)

# CMAKE_MSVCIDE_RUN_PATH

Added in version 3.10.

Extra PATH locations that should be used when executing [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) or [`add_custom_target()`](https://cmake.org/cmake/help/v4.4/command/add_custom_target.html#command:add_custom_target) when using [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators). This allows for running commands and using dll's that the IDE environment is not aware of.

If not set explicitly the value is initialized by the `CMAKE_MSVCIDE_RUN_PATH` environment variable, if set, and otherwise left empty.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
