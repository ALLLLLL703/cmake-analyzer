CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION.html)

# CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION

Added in version 3.4.

Visual Studio Windows Target Platform Version.

When targeting Windows 10 and above, [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2015 and above support specification of a Windows SDK version:

- If [`CMAKE_GENERATOR_PLATFORM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_PLATFORM.html#variable:CMAKE_GENERATOR_PLATFORM) specifies a `version=` field, as documented by [Visual Studio Platform Selection](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_PLATFORM.html#visual-studio-platform-selection), that SDK version is selected.

- Otherwise, if the `WindowsSDKVersion` environment variable is set to an available SDK version, that version is selected. This is intended for use in environments established by `vcvarsall.bat` or similar scripts.

    

  Added in version 3.27: This is enabled by policy [`CMP0149`](https://cmake.org/cmake/help/v4.4/policy/CMP0149.html#policy:CMP0149).

- Otherwise, if [`CMAKE_SYSTEM_VERSION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_VERSION.html#variable:CMAKE_SYSTEM_VERSION) is set to an available SDK version, that version is selected.

    

  Changed in version 3.27: This is disabled by policy [`CMP0149`](https://cmake.org/cmake/help/v4.4/policy/CMP0149.html#policy:CMP0149).

- Otherwise, CMake uses the latest Windows SDK version available.

The chosen Windows target version number is provided in `CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION`. If no Windows 10 SDK is available this value will be empty.

One may set a `CMAKE_WINDOWS_KITS_10_DIR` *environment variable* to an absolute path to tell CMake to look for Windows 10 SDKs in a custom location. The specified directory is expected to contain `Include/10.0.*` directories.

See also [`CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION_MAXIMUM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION_MAXIMUM.html#variable:CMAKE_VS_WINDOWS_TARGET_PLATFORM_VERSION_MAXIMUM).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
