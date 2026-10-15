CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_VS_VERSION_BUILD_NUMBER.html)

# CMAKE_VS_VERSION_BUILD_NUMBER

Added in version 3.26.

Visual Studio version.

[Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) for VS 2017 and above set this variable to the Visual Studio version build number in the format `<major>.<minor>.<date>.<build>`.

The components are:

`<major>.<minor>`

The VS major and minor version numbers. These are the same as the release version numbers.

`<date>`

A build date in the format `MMMDD`, where `MMM` is a month index since an epoch used by Microsoft, and `DD` is a day in that month.

`<build>`

A build index on the day represented by `<date>`.

The build number is reported by `vswhere` as `installationVersion`. For example, VS 16.11.10 has build number `16.11.32126.315`.

See also the [`CMAKE_GENERATOR_INSTANCE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GENERATOR_INSTANCE.html#variable:CMAKE_GENERATOR_INSTANCE) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
