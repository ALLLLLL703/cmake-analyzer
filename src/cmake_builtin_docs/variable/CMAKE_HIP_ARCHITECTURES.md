CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HIP_ARCHITECTURES.html)

# CMAKE_HIP_ARCHITECTURES

Added in version 3.21.

List of GPU architectures to for which to generate device code. Architecture names are interpreted based on [`CMAKE_HIP_PLATFORM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HIP_PLATFORM.html#variable:CMAKE_HIP_PLATFORM).

This is initialized based on the value of [`CMAKE_HIP_PLATFORM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HIP_PLATFORM.html#variable:CMAKE_HIP_PLATFORM):

**`amd`**

Uses architectures reported by `rocm_agent_enumerator`, if available, and otherwise to a default chosen by the compiler.

**`spirv`**

Defaults to `OFF`. The HIP package (e.g., chipStar) handles targeting via its own configuration.

This variable is used to initialize the [`HIP_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/prop_tgt/HIP_ARCHITECTURES.html#prop_tgt:HIP_ARCHITECTURES) property on all targets. See the target property for additional information.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
