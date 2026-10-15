CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_DEFAULT_COMPONENT_NAME.html)

# CMAKE_INSTALL_DEFAULT_COMPONENT_NAME

Default component used in [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) commands.

If an [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) command is used without the `COMPONENT` argument, these files will be grouped into a default component. The name of this default install component will be taken from this variable. It defaults to `Unspecified`.

There is a special component name:

**`<PROJECT_NAME>`**

Added in version 4.4.

This literal placeholder will expand to the contents of the [`PROJECT_NAME`](https://cmake.org/cmake/help/v4.4/variable/PROJECT_NAME.html#variable:PROJECT_NAME) variable when the [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) command is evaluated. If no [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command has been called it defaults to `Unspecified`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
