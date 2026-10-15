CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_NDK_TOOLCHAIN_VERSION.html)

# CMAKE_ANDROID_NDK_TOOLCHAIN_VERSION

Added in version 3.7.

When [Cross Compiling for Android with the NDK](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-the-ndk), this variable may be set to specify the version of the toolchain to be used as the compiler.

On NDK r19 or above, this variable must be unset or set to `clang`.

On NDK r18 or below, this variable must be set to one of these forms:

- `<major>.<minor>`: GCC of specified version

- `clang<major>.<minor>`: Clang of specified version

- `clang`: Clang of most recent available version

A toolchain of the requested version will be selected automatically to match the ABI named in the [`CMAKE_ANDROID_ARCH_ABI`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_ARCH_ABI.html#variable:CMAKE_ANDROID_ARCH_ABI) variable.

If not specified, the default will be a value that selects the latest available GCC toolchain.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
