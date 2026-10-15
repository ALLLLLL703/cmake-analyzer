CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_ARCH.html)

# CMAKE_ANDROID_ARCH

Added in version 3.4.

When [Cross Compiling for Android with NVIDIA Nsight Tegra Visual Studio Edition](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-nvidia-nsight-tegra-visual-studio-edition), this variable may be set to specify the default value for the [`ANDROID_ARCH`](https://cmake.org/cmake/help/v4.4/prop_tgt/ANDROID_ARCH.html#prop_tgt:ANDROID_ARCH) target property. See that target property for additional information.

Otherwise, when [Cross Compiling for Android](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android), this variable provides the name of the Android architecture corresponding to the value of the [`CMAKE_ANDROID_ARCH_ABI`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_ARCH_ABI.html#variable:CMAKE_ANDROID_ARCH_ABI) variable. The architecture name may be one of:

- `arm`

- `arm64`

- `mips`

- `mips64`

- `x86`

- `x86_64`

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
