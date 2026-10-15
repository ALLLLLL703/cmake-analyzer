CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MACOSX_BUNDLE.html)

# CMAKE_MACOSX_BUNDLE

Default value for [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) of targets.

This variable is used to initialize the [`MACOSX_BUNDLE`](https://cmake.org/cmake/help/v4.4/prop_tgt/MACOSX_BUNDLE.html#prop_tgt:MACOSX_BUNDLE) property on all the targets. See that target property for additional information.

This variable is set to `ON` by default if [`CMAKE_SYSTEM_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_NAME.html#variable:CMAKE_SYSTEM_NAME) equals to [iOS, tvOS, visionOS or watchOS](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-ios-tvos-visionos-or-watchos).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
