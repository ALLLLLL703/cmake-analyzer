CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/MSVC_IDE.html)

# MSVC_IDE

`True` when using the Microsoft Visual C++ IDE.

Set to `true` when the target platform is the Microsoft Visual C++ IDE, as opposed to the command line compiler.

Note

This variable is only available after compiler detection has been performed, so it is not available to toolchain files or before the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) call which uses an MSVC-like compiler.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
