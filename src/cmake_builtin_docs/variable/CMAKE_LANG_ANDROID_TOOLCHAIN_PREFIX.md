CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ANDROID_TOOLCHAIN_PREFIX.html)

# CMAKE_\<LANG\>_ANDROID_TOOLCHAIN_PREFIX

Added in version 3.7.

When [Cross Compiling for Android](https://cmake.org/cmake/help/v4.4/manual/cmake-toolchains.7.html#cross-compiling-for-android) this variable contains the absolute path prefixing the toolchain GNU compiler and its binutils.

See also [`CMAKE_<LANG>_ANDROID_TOOLCHAIN_SUFFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ANDROID_TOOLCHAIN_SUFFIX.html#variable:CMAKE_%3CLANG%3E_ANDROID_TOOLCHAIN_SUFFIX) and [`CMAKE_<LANG>_ANDROID_TOOLCHAIN_MACHINE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ANDROID_TOOLCHAIN_MACHINE.html#variable:CMAKE_%3CLANG%3E_ANDROID_TOOLCHAIN_MACHINE).

For example, the path to the linker is:

~~~
${CMAKE_CXX_ANDROID_TOOLCHAIN_PREFIX}ld${CMAKE_CXX_ANDROID_TOOLCHAIN_SUFFIX}
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
