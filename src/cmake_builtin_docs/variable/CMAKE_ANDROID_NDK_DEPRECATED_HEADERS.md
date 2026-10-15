CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_NDK_DEPRECATED_HEADERS.html)

# CMAKE_ANDROID_NDK_DEPRECATED_HEADERS

Added in version 3.9.

When [Cross Compiling for Android with the NDK](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-the-ndk), this variable may be set to specify whether to use the deprecated per-api-level headers instead of the unified headers.

If not specified, the default will be *false* if using a NDK version that provides the unified headers and *true* otherwise.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
