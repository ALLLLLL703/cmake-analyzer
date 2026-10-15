CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_WINRT_BY_DEFAULT.html)

# CMAKE_VS_WINRT_BY_DEFAULT

Added in version 3.13.

Inform [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2010 and above that the target platform enables WinRT compilation by default and it needs to be explicitly disabled if `/ZW` or [`VS_WINRT_COMPONENT`](https://cmake.org/cmake/help/v4.4/prop_tgt/VS_WINRT_COMPONENT.html#prop_tgt:VS_WINRT_COMPONENT) is omitted (as opposed to enabling it when either of those options is present)

This makes cmake configuration consistent in terms of WinRT among platforms - if you did not enable the WinRT compilation explicitly, it will be disabled (by either not enabling it or explicitly disabling it)

Note: WinRT compilation is always explicitly disabled for C language source files, even if it is expliclty enabled for a project

This variable is meant to be set by a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) for such platforms.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
