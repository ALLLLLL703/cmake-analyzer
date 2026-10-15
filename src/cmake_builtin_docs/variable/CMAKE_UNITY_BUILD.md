CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_UNITY_BUILD.html)

# CMAKE_UNITY_BUILD

Added in version 3.16.

This variable is used to initialize the [`UNITY_BUILD`](https://cmake.org/cmake/help/v4.4/prop_tgt/UNITY_BUILD.html#prop_tgt:UNITY_BUILD) property of targets when they are created. Setting it to true enables batch compilation of multiple sources within each target. This feature is known as a *Unity* or *Jumbo* build.

Projects should not set this variable, it is intended as a developer control to be set on the [`cmake(1)`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#manual:cmake(1)) command line or other equivalent methods. The developer must have the ability to enable or disable unity builds according to the capabilities of their own machine and compiler.

By default, this variable is not set, which will result in unity builds being disabled.

Note

This option currently does not work well in combination with the [`CMAKE_EXPORT_COMPILE_COMMANDS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_COMPILE_COMMANDS.html#variable:CMAKE_EXPORT_COMPILE_COMMANDS) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
