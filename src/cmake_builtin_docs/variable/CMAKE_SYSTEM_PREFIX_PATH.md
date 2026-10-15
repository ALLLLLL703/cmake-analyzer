CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html)

# CMAKE_SYSTEM_PREFIX_PATH

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of directories specifying installation *prefixes* to be searched by the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package), [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program), [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library), [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file), and [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path) commands. Each command will add appropriate subdirectories (like `bin`, `lib`, or `include`) as specified in its own documentation.

By default this contains the system directories for the current system, the [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX), and the [`CMAKE_STAGING_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STAGING_PREFIX.html#variable:CMAKE_STAGING_PREFIX). The installation and staging prefixes may be excluded by setting the [`CMAKE_FIND_NO_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_NO_INSTALL_PREFIX.html#variable:CMAKE_FIND_NO_INSTALL_PREFIX) variable before the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) invocation.

The system directories that are contained in `CMAKE_SYSTEM_PREFIX_PATH` are locations that typically include installed software. An example being `/usr/local` for UNIX based platforms. In addition to standard platform locations, CMake will also add values to `CMAKE_SYSTEM_PREFIX_PATH` based on environment variables. The environment variables and search locations that CMake uses may evolve over time, as platforms and their conventions also evolve. The following provides an indicative list of environment variables and locations that CMake searches, but they are subject to change:

**CrayLinuxEnvironment:**

- `ENV{SYSROOT_DIR}/`

- `ENV{SYSROOT_DIR}/usr`

- `ENV{SYSROOT_DIR}/usr/local`

**Darwin:**

- `ENV{SDKROOT}/usr` When `CMAKE_OSX_SYSROOT` is not explicitly specified.

**OpenBSD:**

- `ENV{LOCALBASE}`

**Unix:**

- `ENV{CONDA_PREFIX}` when using a conda compiler

**MSYSTEM environment with MinGW toolchain:**

Added in version 3.28.

- `ENV{MSYSTEM_PREFIX}/local`

- `ENV{MSYSTEM_PREFIX}`

**Windows:**

- `ENV{ProgramW6432}`

- `ENV{ProgramFiles}`

- `ENV{ProgramFiles(x86)}`

- `ENV{SystemDrive}/Program Files`

- `ENV{SystemDrive}/Program Files (x86)`

`CMAKE_SYSTEM_PREFIX_PATH` is *not* intended to be modified by the project; use [`CMAKE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PREFIX_PATH.html#variable:CMAKE_PREFIX_PATH) for this.

See also [`CMAKE_SYSTEM_INCLUDE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_INCLUDE_PATH.html#variable:CMAKE_SYSTEM_INCLUDE_PATH), [`CMAKE_SYSTEM_LIBRARY_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_LIBRARY_PATH.html#variable:CMAKE_SYSTEM_LIBRARY_PATH), [`CMAKE_SYSTEM_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PROGRAM_PATH.html#variable:CMAKE_SYSTEM_PROGRAM_PATH), and [`CMAKE_SYSTEM_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PATH).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
