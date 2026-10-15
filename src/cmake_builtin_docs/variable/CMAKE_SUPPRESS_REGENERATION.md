CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SUPPRESS_REGENERATION.html)

# CMAKE_SUPPRESS_REGENERATION

Added in version 3.12.

If `CMAKE_SUPPRESS_REGENERATION` is `OFF`, which is default, then CMake adds a special target on which all other targets depend that checks the build system and optionally re-runs CMake to regenerate the build system when the target specification source changes.

If this variable evaluates to `ON` at the end of the top-level `CMakeLists.txt` file, CMake will not add the regeneration target to the build system or perform any build system checks.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
