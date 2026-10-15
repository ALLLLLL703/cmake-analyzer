CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_WINDOWS_KMDF_VERSION.html)

# CMAKE_WINDOWS_KMDF_VERSION

Added in version 3.31.

Specify the [Kernel-Mode Drive Framework](https://learn.microsoft.com/en-us/windows-hardware/drivers/wdf/kmdf-version-history) target version.

A [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) that sets [`CMAKE_SYSTEM_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_NAME.html#variable:CMAKE_SYSTEM_NAME) to `WindowsKernelModeDriver` must also set `CMAKE_WINDOWS_KMDF_VERSION` to specify the KMDF target version.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
