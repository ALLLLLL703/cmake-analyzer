CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_REQUIRED.html)

# CMAKE_FIND_REQUIRED

Added in version 4.1.

If enabled, the following commands are treated as having the `REQUIRED` keyword unless provided with the `OPTIONAL` keyword:

- [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)

- [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program)

- [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library)

- [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path)

- [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file)

When [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) loads a `Find<PackageName>.cmake` or `<PackageName>Config.cmake` module, the `CMAKE_FIND_REQUIRED` variable is automatically unset within it to restore the default behavior for nested find operations. The module is free to set the `CMAKE_FIND_REQUIRED` variable itself to opt-in to the behavior.

Note that enabling this variable breaks some commonly used patterns. Multiple calls to [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) are sometimes used to obtain a different search order to the default.

See also the [`CMAKE_REQUIRE_FIND_PACKAGE_<PackageName>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_REQUIRE_FIND_PACKAGE_PackageName.html#variable:CMAKE_REQUIRE_FIND_PACKAGE_%3CPackageName%3E) variable for making a [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) call `REQUIRED`, and for additional information on how enabling these variables can break commonly used patterns.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
