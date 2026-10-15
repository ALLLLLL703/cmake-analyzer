CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_XCODE_GENERATE_TOP_LEVEL_PROJECT_ONLY.html)

# CMAKE_XCODE_GENERATE_TOP_LEVEL_PROJECT_ONLY

Added in version 3.11.

If enabled, the [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator will generate only a single Xcode project file for the topmost [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command instead of generating one for every `project()` command.

This could be useful to speed up the CMake generation step for large projects and to work-around a bug in the `ZERO_CHECK` logic.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
