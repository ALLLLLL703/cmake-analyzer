CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_ARM_MODE.html)

# CMAKE_ANDROID_ARM_MODE

Added in version 3.7.

When [Cross Compiling for Android](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android) and [`CMAKE_ANDROID_ARCH_ABI`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_ARCH_ABI.html#variable:CMAKE_ANDROID_ARCH_ABI) is set to one of the `armeabi` architectures, set `CMAKE_ANDROID_ARM_MODE` to `ON` to target 32-bit ARM processors (`-marm`). Otherwise, the default is to target the 16-bit Thumb processors (`-mthumb`).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
