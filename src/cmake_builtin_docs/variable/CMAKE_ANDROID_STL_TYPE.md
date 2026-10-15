CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ANDROID_STL_TYPE.html)

# CMAKE_ANDROID_STL_TYPE

Added in version 3.4.

When [Cross Compiling for Android with NVIDIA Nsight Tegra Visual Studio Edition](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-nvidia-nsight-tegra-visual-studio-edition), this variable may be set to specify the default value for the [`ANDROID_STL_TYPE`](https://cmake.org/cmake/help/v4.4/prop_tgt/ANDROID_STL_TYPE.html#prop_tgt:ANDROID_STL_TYPE) target property. See that target property for additional information.

When [Cross Compiling for Android with the NDK](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-the-ndk), this variable may be set to specify the STL variant to be used. The value may be one of:

**`none`**

No C++ Support

**`system`**

Minimal C++ without STL

**`gabi++_static`**

GAbi++ Static

**`gabi++_shared`**

GAbi++ Shared

**`gnustl_static`**

GNU libstdc++ Static

**`gnustl_shared`**

GNU libstdc++ Shared

**`c++_static`**

LLVM libc++ Static

**`c++_shared`**

LLVM libc++ Shared

**`stlport_static`**

STLport Static

**`stlport_shared`**

STLport Shared

The default value is `gnustl_static` on NDK versions that provide it and otherwise `c++_static`. Note that this default differs from the native NDK build system because CMake may be used to build projects for Android that are not natively implemented for it and use the C++ standard library.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
