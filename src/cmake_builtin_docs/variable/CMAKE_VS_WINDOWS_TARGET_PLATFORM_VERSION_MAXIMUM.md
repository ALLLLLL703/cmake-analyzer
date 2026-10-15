CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION_MAXIMUM.html)

# CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION_MAXIMUM

Added in version 3.19.

Override the [Windows 10 SDK Maximum Version for VS 2015](https://cmake.org/cmake/help/v4.4/generator/Visual%20Studio%2014%202015.html#windows-10-sdk-maximum-version-for-vs-2015) and beyond.

The `CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION_MAXIMUM` variable may be set to a false value (e.g. `OFF`, `FALSE`, or `0`) or the SDK version to use as the maximum (e.g. `10.0.14393.0`). If unset, the default depends on which version of Visual Studio is targeted by the current generator.

This can be used to exclude Windows SDK versions from consideration for [`CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION.html#variable:CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
