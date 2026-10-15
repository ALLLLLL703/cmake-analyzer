CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_API.html)

# CMAKE_ANDROID_API

Added in version 3.1.

When [Cross Compiling for Android with NVIDIA Nsight Tegra Visual Studio Edition](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-nvidia-nsight-tegra-visual-studio-edition), this variable may be set to specify the default value for the [`ANDROID_API`](https://cmake.org/cmake/help/v4.4/prop_tgt/ANDROID_API.html#prop_tgt:ANDROID_API) target property. See that target property for additional information.

When [Cross Compiling for Android](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android), the [`CMAKE_SYSTEM_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_VERSION.html#variable:CMAKE_SYSTEM_VERSION) variable represents the Android API version number targeted. For historical reasons, if a toolchain file sets `CMAKE_ANDROID_API`, but not `CMAKE_SYSTEM_VERSION`, the latter will be initialized using the former.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
