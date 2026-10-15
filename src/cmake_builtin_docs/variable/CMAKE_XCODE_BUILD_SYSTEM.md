CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_XCODE_BUILD_SYSTEM.html)

# CMAKE_XCODE_BUILD_SYSTEM

Added in version 3.19.

Xcode build system selection.

The [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode) generator defines this variable to indicate which variant of the Xcode build system will be used. The value is the version of Xcode in which the corresponding build system first became mature enough for use by CMake. The possible values are:

**`1`**

The original Xcode build system. This is the default when using Xcode 11.x or below and supported up to Xcode 13.x.

**`12`**

The Xcode "new build system" introduced by Xcode 10. It became mature enough for use by CMake in Xcode 12. This is the default when using Xcode 12.x or above.

The `CMAKE_XCODE_BUILD_SYSTEM` variable is informational and should not be modified by project code. See the [Toolset and Build System Selection](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#xcode-build-system-selection) documentation section to select the Xcode build system.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
